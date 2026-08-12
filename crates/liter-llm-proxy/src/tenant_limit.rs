//! Per-virtual-key rate-limit and budget enforcement.
//!
//! `liter_llm::tower::ModelRateLimitLayer` and `BudgetLayer` (the core Tower
//! layers wired into `service_pool::build_service_stack`) only track
//! per-MODEL limits — there is no per-key/tenant axis in either.
//! `VirtualKeyConfig` parses `rpm`, `tpm`, and `budget_limit` per key but
//! nothing enforced them (see issue #71). This module closes that gap
//! entirely within the proxy crate: [`KeyLimitLayer`] is built once from
//! `ProxyConfig.keys` and shared across every model's Tower stack in
//! `ServicePool`, so a key's rpm/tpm/budget cap applies across all models it
//! can reach, not per model.
//!
//! # Known limitations
//!
//! - `budget_limit` is enforced as a **cumulative, process-lifetime** spend
//!   cap, not a calendar-month cap. `VirtualKeyConfig::budget_limit` maps to
//!   `ResolvedKey::monthly_budget` in `auth::KeyStore::resolve`, but a true
//!   monthly rollover needs persistent state and belongs in a core-crate
//!   budget ledger (`liter_llm::tower::budget` already has an
//!   `InMemoryBudgetLedger` with a `DimensionLimits::per_tenant` axis for
//!   this — wiring `BudgetLayer`/`ServicePool` to use it is a core-crate
//!   change, out of scope here).
//! - Like model definitions, limits are a snapshot taken when `ServicePool`
//!   is built at startup; they do not pick up hot-reloaded
//!   `ProxyConfig.keys` changes without a process restart (`ServicePool` is
//!   never rebuilt on config reload — see `ProxyServer::serve_with_shutdown`
//!   and issue #69, which only covers `auth::KeyStore` reload).
//! - Lookup is by `tenant_id`, matched against `VirtualKeyConfig.key` (the
//!   built-in `auth::KeyStore::resolve` sets `tenant_id == key`, see
//!   `KeyContext::from_config`/`from_resolved`). A custom `KeyResolver`
//!   (`ProxyServer::with_key_resolver`) that maps to unrelated tenant IDs
//!   will not have its keys found here, and enforcement silently no-ops
//!   (fails open) for that resolver.
//! - Requests that never call `LlmRequest::with_tenant_id` — MCP `chat`/
//!   `embed` tool calls in `mcp/tools.rs` currently do not — carry
//!   `tenant_id: None` and are not rate/budget-limited by this layer.

use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use dashmap::DashMap;
use tower::{Layer, Service};

use liter_llm::client::BoxFuture;
use liter_llm::cost;
use liter_llm::error::{LiterLlmError, Result as LlmResult};
use liter_llm::tenant::TenantId;
use liter_llm::tower::types::{LlmRequest, LlmResponse};

use crate::auth::MASTER_TENANT_ID;
use crate::config::VirtualKeyConfig;

/// Per-key limits extracted from `VirtualKeyConfig` at `ServicePool` build time.
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyLimits {
    pub rpm: Option<u32>,
    pub tpm: Option<u64>,
    pub budget_limit: Option<f64>,
}

/// Build the tenant-id -> limits map from the configured virtual keys.
///
/// See the module-level docs for why keying by `VirtualKeyConfig.key` matches
/// `LlmRequest::tenant_id()` for the default resolver.
pub fn build_key_limits(keys: &[VirtualKeyConfig]) -> HashMap<TenantId, KeyLimits> {
    keys.iter()
        .map(|k| {
            (
                TenantId::from(k.key.as_str()),
                KeyLimits {
                    rpm: k.rpm,
                    tpm: k.tpm,
                    budget_limit: k.budget_limit,
                },
            )
        })
        .collect()
}

/// Sliding-window request/token counters and cumulative spend for one tenant.
///
/// Mirrors `liter_llm::tower::rate_limit::ModelRateState`, plus a
/// non-windowed `spend_usd` accumulator for `budget_limit` (see module docs
/// for why this is a cumulative cap rather than a calendar-month one).
struct TenantWindow {
    request_count: u64,
    token_count: u64,
    window_start: Instant,
    spend_usd: f64,
}

impl TenantWindow {
    fn new() -> Self {
        Self {
            request_count: 0,
            token_count: 0,
            window_start: Instant::now(),
            spend_usd: 0.0,
        }
    }

    fn maybe_reset(&mut self, window: Duration) {
        if self.window_start.elapsed() >= window {
            self.request_count = 0;
            self.token_count = 0;
            self.window_start = Instant::now();
        }
    }
}

/// Tower [`Layer`] enforcing per-virtual-key rpm/tpm/budget limits.
///
/// Construct once per `ServicePool` and reuse `.layer(..)` for every model's
/// stack so the underlying counters are shared across models — a key's
/// limits apply globally, not per model.
pub struct KeyLimitLayer {
    limits: Arc<HashMap<TenantId, KeyLimits>>,
    window: Duration,
    state: Arc<DashMap<TenantId, TenantWindow>>,
}

impl KeyLimitLayer {
    /// Create a new layer from a pre-built tenant -> limits map, using the
    /// default 60-second rpm/tpm window.
    pub fn new(limits: Arc<HashMap<TenantId, KeyLimits>>) -> Self {
        Self {
            limits,
            window: Duration::from_secs(60),
            state: Arc::new(DashMap::new()),
        }
    }
}

impl<S> Layer<S> for KeyLimitLayer {
    type Service = KeyLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        KeyLimitService {
            inner,
            limits: Arc::clone(&self.limits),
            window: self.window,
            state: Arc::clone(&self.state),
        }
    }
}

/// Tower [`Service`] produced by [`KeyLimitLayer`].
pub struct KeyLimitService<S> {
    inner: S,
    limits: Arc<HashMap<TenantId, KeyLimits>>,
    window: Duration,
    state: Arc<DashMap<TenantId, TenantWindow>>,
}

impl<S: Clone> Clone for KeyLimitService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            limits: Arc::clone(&self.limits),
            window: self.window,
            state: Arc::clone(&self.state),
        }
    }
}

impl<S> Service<LlmRequest> for KeyLimitService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, LlmResult<LlmResponse>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<LlmResult<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        // ~keep Master-key traffic and requests with no tenant_id at all (e.g.
        // ~keep MCP tool calls, see module docs) are intentionally unlimited here.
        let Some(tenant_id) = req.tenant_id().cloned() else {
            return Box::pin(self.inner.call(req));
        };
        if tenant_id.as_ref() == MASTER_TENANT_ID {
            return Box::pin(self.inner.call(req));
        }
        let Some(limits) = self.limits.get(&tenant_id).copied() else {
            return Box::pin(self.inner.call(req));
        };

        let window = self.window;
        let state = Arc::clone(&self.state);

        {
            let mut entry = state.entry(tenant_id.clone()).or_insert_with(TenantWindow::new);
            entry.maybe_reset(window);

            if let Some(rpm) = limits.rpm
                && entry.request_count >= u64::from(rpm)
            {
                let tenant_id = tenant_id.clone();
                return Box::pin(async move {
                    Err(LiterLlmError::RateLimited {
                        message: format!(
                            "key '{tenant_id}' exceeded {rpm} requests per {:.0}s window",
                            window.as_secs_f64()
                        ),
                        retry_after: Some(window),
                    })
                });
            }

            if let Some(tpm) = limits.tpm
                && entry.token_count >= tpm
            {
                let tenant_id = tenant_id.clone();
                return Box::pin(async move {
                    Err(LiterLlmError::RateLimited {
                        message: format!(
                            "key '{tenant_id}' exceeded {tpm} tokens per {:.0}s window",
                            window.as_secs_f64()
                        ),
                        retry_after: Some(window),
                    })
                });
            }

            if let Some(budget_limit) = limits.budget_limit
                && entry.spend_usd >= budget_limit
            {
                let tenant_id = tenant_id.clone();
                let spend = entry.spend_usd;
                return Box::pin(async move {
                    Err(LiterLlmError::BudgetExceeded {
                        message: format!(
                            "key '{tenant_id}' exceeded its budget: spent ${spend:.6}, limit ${budget_limit:.6}"
                        ),
                        model: None,
                    })
                });
            }

            entry.request_count += 1;
        }

        let model = req.model().unwrap_or("unknown").to_owned();
        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp = fut.await?;

            if let Some(usage) = resp.usage() {
                let total_tokens = usage.prompt_tokens + usage.completion_tokens;
                let usd = cost::completion_cost(&model, usage.prompt_tokens, usage.completion_tokens);
                if let Some(mut entry) = state.get_mut(&tenant_id) {
                    entry.maybe_reset(window);
                    entry.token_count += total_tokens;
                    if let Some(usd) = usd {
                        entry.spend_usd += usd;
                    }
                }
            }

            Ok(resp)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use liter_llm::types::ChatCompletionRequest;

    use super::*;

    /// Sentinel inner service. Its `call` always errors distinctly from
    /// [`LiterLlmError::RateLimited`]/[`LiterLlmError::BudgetExceeded`], so
    /// reaching it (vs. being rejected by [`KeyLimitLayer`] itself) is
    /// observable without needing to fabricate a full `LlmResponse`.
    #[derive(Clone)]
    struct ReachedInner;

    impl Service<LlmRequest> for ReachedInner {
        type Response = LlmResponse;
        type Error = LiterLlmError;
        type Future = BoxFuture<'static, LlmResult<LlmResponse>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<LlmResult<()>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: LlmRequest) -> Self::Future {
            Box::pin(async {
                Err(LiterLlmError::InternalError {
                    message: "reached inner service".to_string(),
                })
            })
        }
    }

    fn chat_request(model: &str) -> LlmRequest {
        LlmRequest::Chat(ChatCompletionRequest {
            model: model.to_string(),
            ..Default::default()
        })
    }

    fn assert_reached_inner(result: LlmResult<LlmResponse>) {
        match result {
            Err(LiterLlmError::InternalError { message }) => {
                assert_eq!(message, "reached inner service");
            }
            other => panic!("expected the request to reach the inner service, got: {other:?}"),
        }
    }

    fn limits_map(entries: &[(&str, KeyLimits)]) -> Arc<HashMap<TenantId, KeyLimits>> {
        Arc::new(entries.iter().map(|(k, v)| (TenantId::from(*k), *v)).collect())
    }

    #[test]
    fn build_key_limits_maps_key_token_to_tenant_id() {
        let keys = vec![VirtualKeyConfig {
            key: "vk-a".to_string(),
            description: None,
            models: vec![],
            rpm: Some(5),
            tpm: Some(1000),
            budget_limit: Some(1.5),
            provider_credentials: vec![],
        }];
        let map = build_key_limits(&keys);
        let limits = map.get(&TenantId::from("vk-a")).expect("key must be present");
        assert_eq!(limits.rpm, Some(5));
        assert_eq!(limits.tpm, Some(1000));
        assert_eq!(limits.budget_limit, Some(1.5));
    }

    #[tokio::test]
    async fn requests_without_tenant_id_are_unlimited() {
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: Some(0),
                tpm: None,
                budget_limit: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        assert_reached_inner(svc.call(chat_request("gpt-4o")).await);
    }

    #[tokio::test]
    async fn master_tenant_is_unlimited() {
        let limits = limits_map(&[(
            MASTER_TENANT_ID,
            KeyLimits {
                rpm: Some(0),
                tpm: None,
                budget_limit: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        let req = chat_request("gpt-4o").with_tenant_id(MASTER_TENANT_ID);
        assert_reached_inner(svc.call(req).await);
    }

    #[tokio::test]
    async fn unconfigured_key_is_unlimited() {
        let limits = limits_map(&[]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        assert_reached_inner(svc.call(chat_request("gpt-4o").with_tenant_id("vk-unknown")).await);
    }

    #[tokio::test]
    async fn rpm_limit_rejects_after_exhausted() {
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: Some(1),
                tpm: None,
                budget_limit: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        assert_reached_inner(svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await);

        let second = svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await;
        assert!(
            matches!(second, Err(LiterLlmError::RateLimited { .. })),
            "second request must exceed rpm=1, got: {second:?}"
        );
    }

    #[tokio::test]
    async fn rpm_limit_is_shared_across_models_for_the_same_key() {
        // ~keep A single `KeyLimitLayer` reused via `.layer(..)` for two
        // ~keep different inner services must share counters (the whole point
        // ~keep of building it once in `ServicePool::from_config`).
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: Some(1),
                tpm: None,
                budget_limit: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc_model_a = layer.layer(ReachedInner);
        let mut svc_model_b = layer.layer(ReachedInner);

        assert_reached_inner(svc_model_a.call(chat_request("model-a").with_tenant_id("vk-a")).await);

        let second = svc_model_b.call(chat_request("model-b").with_tenant_id("vk-a")).await;
        assert!(
            matches!(second, Err(LiterLlmError::RateLimited { .. })),
            "rpm=1 must be exhausted globally for the key, not per model, got: {second:?}"
        );
    }

    #[tokio::test]
    async fn budget_limit_of_zero_rejects_immediately() {
        // ~keep Mirrors the core `BudgetLayer`'s pre-flight `>=` semantics: an
        // ~keep exact-zero cap rejects before any spend is ever recorded.
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: None,
                tpm: None,
                budget_limit: Some(0.0),
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        let result = svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await;
        assert!(
            matches!(result, Err(LiterLlmError::BudgetExceeded { .. })),
            "a zero budget cap must reject immediately, got: {result:?}"
        );
    }
}
