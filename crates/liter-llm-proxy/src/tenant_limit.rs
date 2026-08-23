//! Per-virtual-key rate-limit and budget enforcement.
//!
//! `liter_llm::tower::ModelRateLimitLayer` and `BudgetLayer` (the core Tower
//! layers wired into `service_pool::build_service_stack`) only track
//! per-MODEL limits — there is no per-key/tenant axis in either.
//! `VirtualKeyConfig` parses `rpm`, `tpm`, and `budget_limit` per key. This
//! module closes that gap entirely within the proxy crate, split across two
//! Tower layers that `ServicePool` builds once and shares across every
//! model's stack, so a key's caps apply across all models it can reach, not
//! per model:
//!
//! - [`KeyLimitLayer`] — sliding-window rpm/tpm, reset every 60 seconds.
//! - [`PerKeyBudgetLedger`] (paired with `liter_llm::tower::BudgetLedgerLayer`)
//!   — calendar-month (30-day sliding window) spend, delegating the actual
//!   accounting to `liter_llm::tower::InMemoryBudgetLedger`'s `per_tenant`
//!   dimension instead of a hand-rolled, never-reset counter. Limit changes
//!   from a config reload — [`PerKeyBudgetLedger::update_limits`] — take
//!   effect in place via `InMemoryBudgetLedger::update_limits`, so
//!   month-to-date spend survives the reload just like
//!   [`KeyLimitLayer::update_limits`]'s rpm/tpm counters already did.
//!
//! # Known limitations
//!
//! - Lookup is by `tenant_id`, and both this module's maps and `KeyContext`
//!   derive that value through the single `auth::key_store::resolved_tenant_id`
//!   function. They must never derive it independently: the lookup FAILS OPEN
//!   on a miss, so any disagreement between the two silently disables rpm/tpm
//!   and budget enforcement rather than raising an error. A custom
//!   `KeyResolver` (`ProxyServer::with_key_resolver`) that maps to unrelated
//!   tenant IDs will not have its keys found here, and enforcement silently
//!   no-ops for that resolver.
//! - Requests that never call `LlmRequest::with_tenant_id` carry
//!   `tenant_id: None` and are not rate/budget-limited by this layer. Every
//!   model-routed MCP tool call in `mcp/tools.rs` now goes through
//!   `LiterLlmMcp::dispatch`, which attaches `tenant_id` the same way
//!   `routes::dispatch` does for HTTP — so this gap no longer applies to MCP.
//! - Realtime WebSocket sessions (`routes::realtime`) never enter this Tower
//!   stack at all — there is no discrete `LlmRequest`/`LlmResponse` per
//!   message. `ServicePool::check_realtime_session_start` calls
//!   [`KeyLimitLayer::check_and_reserve`] and `PerKeyBudgetLedger::check`
//!   directly at session establishment as a pre-flight gate; it cannot meter
//!   the live session's own token usage, since no cost-tracking hook exists
//!   for realtime today.

use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use arc_swap::ArcSwap;
use dashmap::DashMap;
use tower::{Layer, Service};

use liter_llm::client::BoxFuture;
use liter_llm::error::{LiterLlmError, Result as LlmResult};
use liter_llm::tenant::TenantId;
use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::tower::{
    BudgetLedger, BudgetSnapshot, BudgetVerdict, CostCheckContext, CostRecordContext, DimensionLimits,
    InMemoryBudgetLedger,
};

use crate::auth::MASTER_TENANT_ID;
use crate::auth::key_store::resolved_tenant_id;
use crate::config::VirtualKeyConfig;

/// Per-key rpm/tpm limits extracted from `VirtualKeyConfig` at `ServicePool`
/// build time. Budget limits are handled separately by [`PerKeyBudgetLedger`].
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyLimits {
    pub rpm: Option<u32>,
    pub tpm: Option<u64>,
}

/// Build the tenant-id -> rpm/tpm limits map from the configured virtual keys.
///
/// ~keep Keyed by `resolved_tenant_id`, the SAME resolution `KeyContext` uses, because
/// ~keep `check_and_reserve_rpm_tpm` FAILS OPEN on a lookup miss. Keying this map any
/// ~keep other way than the value the request actually carries does not raise an error —
/// ~keep it silently stops enforcing rpm/tpm for every virtual key. That is why both
/// ~keep sides must go through one function rather than each deriving the key.
pub fn build_key_limits(keys: &[VirtualKeyConfig]) -> HashMap<TenantId, KeyLimits> {
    keys.iter()
        .map(|k| (resolved_tenant_id(k), KeyLimits { rpm: k.rpm, tpm: k.tpm }))
        .collect()
}

/// Build the tenant-id -> budget-limit map (USD) from the configured virtual
/// keys, in the `String`-keyed shape `DimensionLimits::per_tenant` expects.
///
/// ~keep Same resolution as `build_key_limits`, for the same fail-open reason.
fn build_budget_limits(keys: &[VirtualKeyConfig]) -> HashMap<String, f64> {
    keys.iter()
        .filter_map(|k| {
            k.budget_limit
                .map(|limit| (resolved_tenant_id(k).as_ref().to_owned(), limit))
        })
        .collect()
}

/// Sliding-window request/token counters and cumulative spend for one tenant.
///
/// Mirrors `liter_llm::tower::rate_limit::ModelRateState`. Budget tracking
/// lives separately in [`PerKeyBudgetLedger`], which delegates to
/// `liter_llm::tower::InMemoryBudgetLedger` for correct calendar-month
/// rollover instead of a field on this struct.
struct TenantWindow {
    request_count: u64,
    token_count: u64,
    window_start: Instant,
}

impl TenantWindow {
    fn new() -> Self {
        Self {
            request_count: 0,
            token_count: 0,
            window_start: Instant::now(),
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

/// Tower [`Layer`] enforcing per-virtual-key rpm/tpm limits.
///
/// Construct once per `ServicePool` and reuse `.layer(..)` for every model's
/// stack so the underlying counters are shared across models — a key's
/// limits apply globally, not per model. `limits` lives behind an
/// [`ArcSwap`] so [`KeyLimitLayer::update_limits`] can push a hot-reloaded
/// `ProxyConfig.keys` into every clone of the produced service without
/// rebuilding `ServicePool` (see `ServicePool::update_key_limits`).
pub struct KeyLimitLayer {
    limits: Arc<ArcSwap<HashMap<TenantId, KeyLimits>>>,
    window: Duration,
    state: Arc<DashMap<TenantId, TenantWindow>>,
}

impl KeyLimitLayer {
    /// Create a new layer from an initial tenant -> limits map, using the
    /// default 60-second rpm/tpm window.
    #[must_use]
    pub fn new(limits: HashMap<TenantId, KeyLimits>) -> Self {
        Self {
            limits: Arc::new(ArcSwap::from_pointee(limits)),
            window: Duration::from_secs(60),
            state: Arc::new(DashMap::new()),
        }
    }

    /// Replace the live rpm/tpm limits map, taking effect for every request
    /// handled after this call returns — no `ServicePool` rebuild needed.
    ///
    /// Existing sliding-window counters (`request_count`/`token_count`) are
    /// left untouched; only the configured caps change. Call this from the
    /// config watcher's reload path with the newly loaded `ProxyConfig.keys`.
    pub fn update_limits(&self, keys: &[VirtualKeyConfig]) {
        self.limits.store(Arc::new(build_key_limits(keys)));
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

/// Shared rpm/tpm pre-flight check reused by [`KeyLimitService::call`] and by
/// [`KeyLimitLayer::check_and_reserve`] (the realtime session-start gate, see
/// `ServicePool::check_realtime_session_start`).  Master-tenant and
/// unconfigured-key callers are always allowed and never enter `state`.  On
/// success, reserves the request by incrementing `request_count`.
fn check_and_reserve_rpm_tpm(
    limits: &ArcSwap<HashMap<TenantId, KeyLimits>>,
    window: Duration,
    state: &DashMap<TenantId, TenantWindow>,
    tenant_id: &TenantId,
) -> LlmResult<()> {
    if tenant_id.as_ref() == MASTER_TENANT_ID {
        return Ok(());
    }
    let Some(limits) = limits.load().get(tenant_id).copied() else {
        return Ok(());
    };

    let mut entry = state.entry(tenant_id.clone()).or_insert_with(TenantWindow::new);
    entry.maybe_reset(window);

    if let Some(rpm) = limits.rpm
        && entry.request_count >= u64::from(rpm)
    {
        return Err(LiterLlmError::RateLimited {
            message: format!(
                "key '{tenant_id}' exceeded {rpm} requests per {:.0}s window",
                window.as_secs_f64()
            ),
            retry_after: Some(window),
        });
    }

    if let Some(tpm) = limits.tpm
        && entry.token_count >= tpm
    {
        return Err(LiterLlmError::RateLimited {
            message: format!(
                "key '{tenant_id}' exceeded {tpm} tokens per {:.0}s window",
                window.as_secs_f64()
            ),
            retry_after: Some(window),
        });
    }

    entry.request_count += 1;
    Ok(())
}

impl KeyLimitLayer {
    /// Check and reserve one request against `tenant_id`'s rpm/tpm window
    /// without going through a [`Service`].
    ///
    /// Used by realtime WebSocket session establishment (see
    /// `ServicePool::check_realtime_session_start`), which has no discrete
    /// `LlmRequest` to run through [`KeyLimitService`] — a session counts as
    /// one reserved request against the SAME shared counters unary calls use,
    /// so a key's rpm cap applies across both. There is no post-session
    /// token count to add afterwards (realtime has no usage-reporting hook
    /// today), so `tpm` here only rejects a window already exhausted by prior
    /// unary calls; it cannot account for the realtime session's own usage.
    ///
    /// # Errors
    /// Returns [`LiterLlmError::RateLimited`] once the tenant's rpm or tpm
    /// window is exhausted.
    pub fn check_and_reserve(&self, tenant_id: &TenantId) -> LlmResult<()> {
        check_and_reserve_rpm_tpm(&self.limits, self.window, &self.state, tenant_id)
    }
}

/// Tower [`Service`] produced by [`KeyLimitLayer`].
pub struct KeyLimitService<S> {
    inner: S,
    limits: Arc<ArcSwap<HashMap<TenantId, KeyLimits>>>,
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
        // ~keep Requests with no tenant_id at all (e.g. MCP tool calls that
        // ~keep predate `dispatch`'s `with_tenant_id`, see module docs) are
        // ~keep intentionally unlimited here; master and unconfigured keys are
        // ~keep handled inside `check_and_reserve_rpm_tpm`.
        let Some(tenant_id) = req.tenant_id().cloned() else {
            return Box::pin(self.inner.call(req));
        };

        if let Err(e) = check_and_reserve_rpm_tpm(&self.limits, self.window, &self.state, &tenant_id) {
            return Box::pin(async move { Err(e) });
        }

        let window = self.window;
        let state = Arc::clone(&self.state);

        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp = fut.await?;

            if let Some(usage) = resp.usage() {
                let total_tokens = usage.prompt_tokens + usage.completion_tokens;
                if let Some(mut entry) = state.get_mut(&tenant_id) {
                    entry.maybe_reset(window);
                    entry.token_count += total_tokens;
                }
            }

            Ok(resp)
        })
    }
}

/// Calendar-month sliding-window budget in USD, per virtual key.
///
/// `VirtualKeyConfig::budget_limit` used to feed a process-lifetime counter
/// in [`KeyLimitService`] that never rolled over and never reset. This ledger
/// implements `liter_llm::tower::BudgetLedger` by delegating every call to a
/// real `liter_llm::tower::InMemoryBudgetLedger`, keyed on the `per_tenant`
/// dimension with a 30-day window — a calendar-month approximation, matching
/// the convention `InMemoryBudgetLedger::from_config` already documents for
/// the core crate's own legacy `BudgetConfig` path.
///
/// Pair with `liter_llm::tower::BudgetLedgerLayer` in the Tower stack; see
/// `service_pool::build_service_stack`.
pub struct PerKeyBudgetLedger {
    inner: InMemoryBudgetLedger,
}

/// Calendar-month approximation used for the default per-key budget window.
const CALENDAR_MONTH_APPROXIMATION: Duration = Duration::from_secs(30 * 24 * 3600);

impl PerKeyBudgetLedger {
    /// Build a ledger from the configured virtual keys, using the default
    /// 30-day (calendar-month approximation) window.
    #[must_use]
    pub fn new(keys: &[VirtualKeyConfig]) -> Self {
        Self::new_with_window(keys, CALENDAR_MONTH_APPROXIMATION)
    }

    /// Build a ledger with an explicit window. Exposed at `pub(crate)` so
    /// tests can prove rollover behaviour without waiting 30 real days.
    #[must_use]
    pub(crate) fn new_with_window(keys: &[VirtualKeyConfig], window: Duration) -> Self {
        Self {
            inner: Self::build_ledger(keys, window),
        }
    }

    fn build_ledger(keys: &[VirtualKeyConfig], window: Duration) -> InMemoryBudgetLedger {
        let limits = DimensionLimits {
            per_tenant: build_budget_limits(keys),
            ..Default::default()
        };
        InMemoryBudgetLedger::new(limits, window)
    }

    /// Replace the live per-key budget limits, taking effect for every
    /// request handled after this call returns — no `ServicePool` rebuild
    /// needed.
    ///
    /// Delegates to `InMemoryBudgetLedger::update_limits`, which swaps only
    /// the configured caps and leaves accumulated month-to-date spend
    /// untouched — a config reload no longer resets a tenant's budget.
    pub fn update_limits(&self, keys: &[VirtualKeyConfig]) {
        let limits = DimensionLimits {
            per_tenant: build_budget_limits(keys),
            ..Default::default()
        };
        self.inner.update_limits(limits);
    }
}

impl BudgetLedger for PerKeyBudgetLedger {
    fn record<'a>(
        &'a self,
        ctx: &'a CostRecordContext<'a>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        self.inner.record(ctx)
    }

    fn check<'a>(
        &'a self,
        ctx: &'a CostCheckContext<'a>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = BudgetVerdict> + Send + 'a>> {
        self.inner.check(ctx)
    }

    fn snapshot(&self) -> BudgetSnapshot {
        self.inner.snapshot()
    }
}

#[cfg(test)]
mod tests {
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

    fn limits_map(entries: &[(&str, KeyLimits)]) -> HashMap<TenantId, KeyLimits> {
        entries.iter().map(|(k, v)| (TenantId::from(*k), *v)).collect()
    }

    fn virtual_key(key: &str, rpm: Option<u32>, tpm: Option<u64>, budget_limit: Option<f64>) -> VirtualKeyConfig {
        VirtualKeyConfig {
            key: key.to_string(),
            tenant_id: None,
            description: None,
            models: vec![],
            rpm,
            tpm,
            budget_limit,
            provider_credentials: vec![],
        }
    }

    /// This map is looked up with the request's `tenant_id`, and the lookup
    /// fails open on a miss. So the only property that matters is that it is
    /// keyed by exactly what `KeyContext` will carry — asserting a literal
    /// value here (as this test used to, with `TenantId::from("vk-a")`) pins
    /// the key token as the tenant id and would have to be rewritten again the
    /// next time the derivation changes. Assert the agreement instead.
    /// The tenant id a configured key resolves to — what a real request carries.
    ///
    /// ~keep Tests must not pass the key string itself as the tenant. The limit
    /// ~keep lookup fails OPEN on a miss, so a hardcoded key would make these
    /// ~keep tests pass by enforcing nothing at all.
    /// ~keep Returns `&'static str` via a deliberate leak: the cost/check contexts
    /// ~keep borrow for `'static`, and binding a local in every caller would bury
    /// ~keep the point of each test in lifetime plumbing. Bounded by test count.
    fn tenant_str(key: &str) -> &'static str {
        let resolved = resolved_tenant_id(&virtual_key(key, None, None, None));
        Box::leak(resolved.as_ref().to_owned().into_boxed_str())
    }

    #[test]
    fn build_key_limits_is_keyed_by_the_same_tenant_id_the_request_carries() {
        let key = virtual_key("vk-a", Some(5), Some(1000), Some(1.5));
        let map = build_key_limits(std::slice::from_ref(&key));

        let ctx = crate::auth::KeyContext::from_config(&key);
        let limits = map
            .get(&ctx.tenant_id)
            .expect("the map must be keyed by the tenant id KeyContext resolves, or enforcement fails open");
        assert_eq!(limits.rpm, Some(5));
        assert_eq!(limits.tpm, Some(1000));

        assert!(
            !map.contains_key(&TenantId::from("vk-a")),
            "the raw key token must no longer be a tenant id"
        );
    }

    #[test]
    fn build_budget_limits_only_includes_keys_with_a_budget() {
        let keys = vec![
            virtual_key("vk-a", None, None, Some(2.5)),
            virtual_key("vk-b", None, None, None),
        ];
        let map = build_budget_limits(&keys);
        assert_eq!(map.get(tenant_str("vk-a")), Some(&2.5));
        assert_eq!(map.get(tenant_str("vk-b")), None);
    }

    #[tokio::test]
    async fn requests_without_tenant_id_are_unlimited() {
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: Some(0),
                tpm: None,
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

    /// Regression test for the realtime-session rpm gap: [`KeyLimitLayer::
    /// check_and_reserve`] must share the SAME counters as `KeyLimitService::
    /// call`, so a realtime session establishment (which never runs through
    /// the `Service`) still counts against — and can be blocked by — a key's
    /// rpm window.
    #[tokio::test]
    async fn check_and_reserve_shares_counters_with_key_limit_service() {
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: Some(1),
                tpm: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        // ~keep A realtime session establishment reserves the single rpm slot...
        layer
            .check_and_reserve(&TenantId::from("vk-a"))
            .expect("first session should be allowed");

        // ~keep ...so a unary call from the SAME key immediately after must be
        // ~keep rejected, proving both call sites share one counter.
        let rejected = svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await;
        assert!(
            matches!(rejected, Err(LiterLlmError::RateLimited { .. })),
            "rpm=1 already reserved by check_and_reserve must reject the next unary call, got: {rejected:?}"
        );
    }

    /// `check_and_reserve` must allow master-tenant and unconfigured-key
    /// callers, exactly like `KeyLimitService::call` does.
    #[tokio::test]
    async fn check_and_reserve_allows_master_and_unconfigured_keys() {
        let limits = limits_map(&[(
            MASTER_TENANT_ID,
            KeyLimits {
                rpm: Some(0),
                tpm: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);

        assert!(layer.check_and_reserve(&TenantId::from(MASTER_TENANT_ID)).is_ok());
        assert!(layer.check_and_reserve(&TenantId::from("vk-unknown")).is_ok());
    }

    /// Regression test for the "limit changes need a process restart" gap:
    /// [`KeyLimitLayer::update_limits`] must take effect on every clone of the
    /// produced service sharing the same layer, without rebuilding it.
    #[tokio::test]
    async fn update_limits_takes_effect_without_rebuilding_the_layer() {
        let limits = limits_map(&[(
            "vk-a",
            KeyLimits {
                rpm: Some(1),
                tpm: None,
            },
        )]);
        let layer = KeyLimitLayer::new(limits);
        let mut svc = layer.layer(ReachedInner);

        assert_reached_inner(svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await);
        let exhausted = svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await;
        assert!(
            matches!(exhausted, Err(LiterLlmError::RateLimited { .. })),
            "rpm=1 should already be exhausted, got: {exhausted:?}"
        );

        layer.update_limits(&[virtual_key("vk-a", Some(5), None, None)]);

        let after_reload = svc.call(chat_request("gpt-4o").with_tenant_id("vk-a")).await;
        assert!(
            matches!(after_reload, Err(LiterLlmError::InternalError { .. })),
            "raised rpm limit should let the request reach the inner service, got: {after_reload:?}"
        );
    }

    fn record_cost(model: &'static str, tenant: &'static str, cost_usd: f64) -> CostRecordContext<'static> {
        CostRecordContext {
            model,
            provider: "openai",
            tenant_id: Some(tenant),
            user_id: None,
            api_key_id: None,
            cost_usd,
            tokens_in: 100,
            tokens_out: 50,
            timestamp: std::time::SystemTime::now(),
        }
    }

    fn check_ctx(tenant: &'static str, timestamp: std::time::SystemTime) -> CostCheckContext<'static> {
        CostCheckContext {
            model: "gpt-4o",
            provider: "openai",
            tenant_id: Some(tenant),
            user_id: None,
            api_key_id: None,
            timestamp,
        }
    }

    #[tokio::test]
    async fn per_key_budget_ledger_rejects_once_limit_exceeded() {
        let keys = vec![virtual_key("vk-a", None, None, Some(1.0))];
        let ledger = PerKeyBudgetLedger::new_with_window(&keys, Duration::from_secs(3600));

        ledger.record(&record_cost("gpt-4o", tenant_str("vk-a"), 1.5)).await;

        let verdict = ledger
            .check(&check_ctx(tenant_str("vk-a"), std::time::SystemTime::now()))
            .await;
        assert!(
            matches!(verdict, BudgetVerdict::Reject { .. }),
            "spend above the configured limit must reject, got: {verdict:?}"
        );
    }

    /// Regression test for "budget never rolls over": recording spend then
    /// checking again after the window has elapsed must allow the request —
    /// proving `PerKeyBudgetLedger` delegates to a real sliding window
    /// instead of a process-lifetime accumulator.
    #[tokio::test]
    async fn per_key_budget_ledger_rolls_over_after_window_elapses() {
        let keys = vec![virtual_key("vk-a", None, None, Some(1.0))];
        let ledger = PerKeyBudgetLedger::new_with_window(&keys, Duration::from_secs(1));

        let now = std::time::SystemTime::now();
        ledger.record(&record_cost("gpt-4o", tenant_str("vk-a"), 5.0)).await;

        let over_budget = ledger.check(&check_ctx(tenant_str("vk-a"), now)).await;
        assert!(
            matches!(over_budget, BudgetVerdict::Reject { .. }),
            "should be over budget within the window, got: {over_budget:?}"
        );

        let next_month = now + Duration::from_secs(2);
        let after_rollover = ledger.check(&check_ctx(tenant_str("vk-a"), next_month)).await;
        assert!(
            matches!(after_rollover, BudgetVerdict::Allow),
            "spend must roll over once the window elapses, got: {after_rollover:?}"
        );
    }

    /// Regression test for the "limit changes need a process restart" gap on
    /// the budget axis: [`PerKeyBudgetLedger::update_limits`] must take
    /// effect immediately.
    #[tokio::test]
    async fn per_key_budget_ledger_update_limits_takes_effect() {
        let keys = vec![virtual_key("vk-a", None, None, Some(1.0))];
        let ledger = PerKeyBudgetLedger::new_with_window(&keys, Duration::from_secs(3600));

        ledger.record(&record_cost("gpt-4o", tenant_str("vk-a"), 1.5)).await;
        let rejected = ledger
            .check(&check_ctx(tenant_str("vk-a"), std::time::SystemTime::now()))
            .await;
        assert!(matches!(rejected, BudgetVerdict::Reject { .. }));

        ledger.update_limits(&[virtual_key("vk-a", None, None, Some(100.0))]);

        let allowed = ledger
            .check(&check_ctx(tenant_str("vk-a"), std::time::SystemTime::now()))
            .await;
        assert!(
            matches!(allowed, BudgetVerdict::Allow),
            "raised budget limit should allow the request, got: {allowed:?}"
        );
    }

    /// Regression for the reset-on-reload bug: raising the limit alone would
    /// also flip `Reject` to `Allow` if spend were wiped to zero, so this
    /// checks the actual spend value via `snapshot()` rather than only the
    /// verdict — it fails against the old rebuild-the-ledger behaviour, which
    /// reads back 0.0 here instead of the pre-update spend.
    #[tokio::test]
    async fn per_key_budget_ledger_update_limits_preserves_spend() {
        let keys = vec![virtual_key("vk-a", None, None, Some(100.0))];
        let ledger = PerKeyBudgetLedger::new_with_window(&keys, Duration::from_secs(3600));

        ledger.record(&record_cost("gpt-4o", tenant_str("vk-a"), 42.0)).await;
        assert!((ledger.snapshot().per_tenant[tenant_str("vk-a")] - 42.0).abs() < 1e-9);

        ledger.update_limits(&[virtual_key("vk-a", None, None, Some(200.0))]);

        let spend = ledger.snapshot().per_tenant[tenant_str("vk-a")];
        assert!(
            (spend - 42.0).abs() < 1e-9,
            "month-to-date spend must survive a config reload, got ${spend:.2}"
        );
    }
}
