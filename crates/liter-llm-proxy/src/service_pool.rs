use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tower::Layer;

use liter_llm::client::{ClientConfigBuilder, DefaultClient};
use liter_llm::error::LiterLlmError;
use liter_llm::observability::{MultiUsageSink, UsageSinkErased};
use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::tower::{
    BudgetConfig, BudgetLayer, BudgetLedgerLayer, BudgetState, CacheConfig, CacheLayer, CooldownLayer,
    CostTrackingLayer, Enforcement, HealthCheckLayer, HooksLayer, LlmService, ModelRateLimitLayer, RateLimitConfig,
    Router, RoutingStrategy, TracingLayer,
};

use crate::config::{ModelEntry, ProxyConfig, VirtualKeyConfig};
use crate::error::ProxyError;
use crate::tenant_limit::{KeyLimitLayer, PerKeyBudgetLedger, build_key_limits};

type Bcs = tower::util::BoxCloneService<LlmRequest, LlmResponse, LiterLlmError>;

/// Thread-safe wrapper around `BoxCloneService`.
///
/// Tower's `BoxCloneService` is `Send` but not `Sync`, because `Service::call`
/// takes `&mut self`. We wrap it in a `Mutex` and clone on each request — the
/// lock is held only for the duration of `Clone::clone` (a handful of `Arc`
/// ref-count bumps).
struct SyncBoxService {
    inner: Mutex<Bcs>,
}

impl SyncBoxService {
    /// Clone the inner service out of the mutex.
    ///
    /// # Errors
    ///
    /// Returns `ProxyError::internal` if the mutex is poisoned.
    fn clone_service(&self) -> Result<Bcs, ProxyError> {
        self.inner
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| ProxyError::internal("service mutex poisoned"))
    }
}

/// A pool of Tower service stacks, one per configured model name.
///
/// Each model name maps to a type-erased `BoxCloneService` with the full
/// middleware stack applied (cache, health check, cooldown, rate limit, cost
/// tracking, budget, tracing).
pub struct ServicePool {
    /// Model name -> Tower service stack.
    services: HashMap<String, SyncBoxService>,
    /// Model name -> raw `DefaultClient` (for File/Batch/Response operations
    /// that bypass the Tower stack).
    clients: HashMap<String, Arc<DefaultClient>>,
    /// The first client inserted during construction, for deterministic
    /// `first_client()` behaviour regardless of `HashMap` iteration order.
    default_client: Option<Arc<DefaultClient>>,
    /// Shared per-key rpm/tpm limiter, retained so a config reload can push
    /// updated limits into every model's Tower stack (see
    /// [`ServicePool::update_key_limits`]) without rebuilding the pool.
    key_limit_layer: Arc<KeyLimitLayer>,
    /// Shared per-key calendar-month budget ledger, retained for the same
    /// reason as `key_limit_layer`.
    budget_ledger: Arc<PerKeyBudgetLedger>,
}

// ~keep SAFETY: `SyncBoxService` wraps a `Mutex<BoxCloneService>` which is `Send + Sync`.
// ~keep `Arc<DefaultClient>` is `Send + Sync`. The compiler verifies these bounds.

impl ServicePool {
    /// Build a pool from the proxy configuration.
    ///
    /// Groups `config.models` by `name` and creates a Tower service stack for
    /// each unique model name.  When multiple deployments share a name they
    /// form an active-active pool dispatched via [`Router`] with
    /// [`RoutingStrategy::RoundRobin`] (see [`build_base_service`]); a single
    /// entry gets a bare `LlmService` with no routing overhead.
    ///
    /// `usage_sink`, when `Some`, is wired into `HooksLayer` outermost in
    /// every model's Tower stack so all completions emit a `UsageEvent`.
    ///
    /// # Errors
    ///
    /// Returns an error string if a `DefaultClient` cannot be constructed for
    /// any model entry.
    pub fn from_config(config: &ProxyConfig, usage_sink: Option<Arc<dyn UsageSinkErased>>) -> Result<Self, String> {
        let mut grouped: HashMap<String, Vec<&ModelEntry>> = HashMap::new();
        for entry in &config.models {
            grouped.entry(entry.name.clone()).or_default().push(entry);
        }

        // ~keep Built once and reused via `.layer(..)` for every model below so a
        // ~keep key's rpm/tpm/budget limit is shared across all models it can
        // ~keep reach, not reset per model (see `tenant_limit` module docs).
        // ~keep Both are Arc-wrapped and retained on `ServicePool` (not just the
        // ~keep local `.layer(..)` closures) so a config reload can call
        // ~keep `update_key_limits` afterwards — see that method's docs.
        let key_limit_layer = Arc::new(KeyLimitLayer::new(build_key_limits(&config.keys)));
        let budget_ledger = Arc::new(PerKeyBudgetLedger::new(&config.keys));

        let mut services = HashMap::new();
        let mut clients = HashMap::new();
        let mut default_client: Option<Arc<DefaultClient>> = None;

        for (name, entries) in &grouped {
            let (base, client_arc) = build_base_service(entries, config)?;

            if default_client.is_none() {
                default_client = Some(Arc::clone(&client_arc));
            }

            let svc = build_service_stack(config, base, usage_sink.clone(), &key_limit_layer, &budget_ledger);

            services.insert(name.clone(), SyncBoxService { inner: Mutex::new(svc) });
            clients.insert(name.clone(), client_arc);
        }

        Ok(Self {
            services,
            clients,
            default_client,
            key_limit_layer,
            budget_ledger,
        })
    }

    /// Push updated per-key rpm/tpm/budget limits from a reloaded
    /// `ProxyConfig.keys` into every model's live Tower stack.
    ///
    /// Both the rpm/tpm limiter and the budget ledger are shared (via `Arc`)
    /// across every model's stack, so this takes effect for all in-flight and
    /// future requests immediately — no `ServicePool` rebuild needed. Call
    /// this from the config watcher's reload path alongside
    /// `auth::KeyStore::reload`.
    ///
    /// See [`PerKeyBudgetLedger::update_limits`] for why updating the budget
    /// axis resets month-to-date spend (rpm/tpm counters are unaffected).
    pub fn update_key_limits(&self, keys: &[VirtualKeyConfig]) {
        self.key_limit_layer.update_limits(keys);
        self.budget_ledger.update_limits(keys);
    }

    /// Clone and return a Tower service stack for the given model name.
    ///
    /// # Errors
    ///
    /// Returns `ProxyError::not_found` if no model with that name exists.
    pub fn get_service(&self, model: &str) -> Result<Bcs, ProxyError> {
        self.services
            .get(model)
            .ok_or_else(|| ProxyError::not_found(format!("model '{model}' not found")))?
            .clone_service()
    }

    /// Return a reference to the raw `DefaultClient` for the given model.
    ///
    /// Useful for File, Batch, and Response API operations that bypass the
    /// Tower middleware stack.
    ///
    /// # Errors
    ///
    /// Returns `ProxyError::not_found` if no model with that name exists.
    pub fn get_client(&self, model: &str) -> Result<Arc<DefaultClient>, ProxyError> {
        self.clients
            .get(model)
            .cloned()
            .ok_or_else(|| ProxyError::not_found(format!("model '{model}' not found")))
    }

    /// Return the first available raw client.
    ///
    /// Used by File, Batch, and Response API endpoints that do not carry a
    /// model field in the request body.
    pub fn first_client(&self) -> Result<Arc<DefaultClient>, ProxyError> {
        self.default_client
            .clone()
            .ok_or_else(|| ProxyError::service_unavailable("no models configured"))
    }

    /// Return the names of all available models.
    pub fn model_names(&self) -> Vec<&str> {
        self.services.keys().map(String::as_str).collect()
    }

    /// Return `true` if the pool contains at least one service.
    pub fn has_any_service(&self) -> bool {
        !self.services.is_empty()
    }
}

/// Extract each entry's real provider model identifier, in the same order as
/// `entries`.
///
/// This is the `Vec` handed to [`Router::with_deployment_models`] — its index
/// must align 1:1 with the `deployments` vec passed to [`Router::new`], since
/// [`RoutingStrategy::Semantic`] resolves a classifier verdict back to a
/// deployment by matching against this list. [`build_base_service`] builds
/// both vecs from the same iteration so they cannot drift apart.
fn deployment_model_ids(entries: &[&ModelEntry]) -> Vec<String> {
    entries.iter().map(|entry| entry.provider_model.clone()).collect()
}

/// Build the base (pre-middleware) service for one model-name group.
///
/// A `name` may be declared by more than one `[[models]]` entry to form an
/// active-active deployment pool (see `proxy-configuration.mdx`). A single
/// entry gets a bare `LlmService` with no routing overhead; multiple entries
/// are wrapped in a [`Router`] using [`RoutingStrategy::RoundRobin`] so every
/// configured deployment actually receives traffic, with
/// [`Router::with_deployment_models`] wired to each entry's real
/// `provider_model` so a future classifier-driven `RoutingStrategy::Semantic`
/// resolves correctly.
///
/// Returns the base service plus the first entry's raw client, which callers
/// use for File/Batch/Response operations that bypass the Tower stack (see
/// `ServicePool::get_client`).
///
/// # Errors
///
/// Returns an error string if any entry's `DefaultClient` fails to build, or
/// if `Router::new` rejects the deployment list (only possible if `entries`
/// is empty, which cannot happen here since callers derive `entries` from a
/// non-empty group).
fn build_base_service(entries: &[&ModelEntry], config: &ProxyConfig) -> Result<(Bcs, Arc<DefaultClient>), String> {
    if entries.len() == 1 {
        let client_arc = Arc::new(build_client(entries[0], config)?);
        let base: Bcs = tower::util::BoxCloneService::new(LlmService::new_from_arc(Arc::clone(&client_arc)));
        return Ok((base, client_arc));
    }

    let mut deployments = Vec::with_capacity(entries.len());
    let mut first_client: Option<Arc<DefaultClient>> = None;
    for entry in entries.iter().copied() {
        let client_arc = Arc::new(build_client(entry, config)?);
        if first_client.is_none() {
            first_client = Some(Arc::clone(&client_arc));
        }
        deployments.push(LlmService::new_from_arc(client_arc));
    }
    let deployment_models = deployment_model_ids(entries);

    let router = Router::new(deployments, RoutingStrategy::RoundRobin)
        .map_err(|e| format!("failed to build router for deployment pool: {e}"))?
        .with_deployment_models(deployment_models);

    let base: Bcs = tower::util::BoxCloneService::new(router);
    let client_arc = first_client.ok_or_else(|| "deployment pool must have at least one entry".to_string())?;
    Ok((base, client_arc))
}

/// Build a `DefaultClient` from a `ModelEntry` and global config defaults.
fn build_client(entry: &ModelEntry, config: &ProxyConfig) -> Result<DefaultClient, String> {
    let api_key = entry.api_key.as_deref().unwrap_or("");

    let mut builder = ClientConfigBuilder::new(api_key);

    if let Some(ref url) = entry.base_url {
        builder = builder.base_url(url);
    }

    let timeout_secs = entry.timeout_secs.unwrap_or(config.general.default_timeout_secs);
    builder = builder.timeout(Duration::from_secs(timeout_secs));
    builder = builder.max_retries(config.general.max_retries);

    let client_config = builder.build();

    DefaultClient::new(client_config, Some(&entry.provider_model))
        .map_err(|e| format!("failed to build client for model '{}': {e}", entry.name))
}

/// Compose the Tower middleware stack, following the same layering order as
/// `managed.rs:build_service_stack`:
///
/// 1. Cache (innermost)
/// 2. HealthCheck
/// 3. Cooldown
/// 4. RateLimit (per-model)
/// 5. KeyLimit (per-virtual-key rpm/tpm — see issue #71)
/// 6. Per-key budget ledger (calendar-month, per-virtual-key — see issue #71)
/// 7. CostTracking
/// 8. Budget (global/per-model, from `[budget]`)
/// 9. Tracing
/// 10. HooksLayer with usage sink (outermost, conditional on `usage_sink.is_some()`)
///
/// HooksLayer sits outermost so it observes every request regardless of which
/// inner layer produces the response (cache hit or live upstream).
fn build_service_stack(
    config: &ProxyConfig,
    base: Bcs,
    usage_sink: Option<Arc<dyn UsageSinkErased>>,
    key_limit_layer: &KeyLimitLayer,
    budget_ledger: &Arc<PerKeyBudgetLedger>,
) -> Bcs {
    let mut svc: Bcs = base;

    if let Some(ref cache_cfg) = config.cache {
        let max_entries = cache_cfg.max_entries.unwrap_or(256);
        let ttl = Duration::from_secs(cache_cfg.ttl_seconds.unwrap_or(300));
        let tower_cache_cfg = CacheConfig {
            max_entries,
            ttl,
            backend: liter_llm::tower::CacheBackend::Memory,
        };
        let layer = CacheLayer::new(tower_cache_cfg);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    if let Some(ref health_cfg) = config.health
        && let Some(interval_secs) = health_cfg.interval_secs
    {
        let layer = HealthCheckLayer::new(Duration::from_secs(interval_secs));
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    if let Some(ref cooldown_cfg) = config.cooldown {
        let layer = CooldownLayer::new(Duration::from_secs(cooldown_cfg.duration_secs));
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    if let Some(ref rl_cfg) = config.rate_limit {
        let tower_rl_cfg = RateLimitConfig {
            rpm: rl_cfg.rpm,
            tpm: rl_cfg.tpm,
            window: Duration::from_secs(60),
        };
        let layer = ModelRateLimitLayer::new(tower_rl_cfg);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    svc = tower::util::BoxCloneService::new(key_limit_layer.layer(svc));

    // ~keep Hard enforcement: mirrors the pre-existing `KeyLimitService` budget
    // ~keep check it replaces, which always rejected (no soft mode for per-key budgets).
    let budget_ledger_layer = BudgetLedgerLayer::new(Arc::clone(budget_ledger), Enforcement::Hard);
    svc = tower::util::BoxCloneService::new(budget_ledger_layer.layer(svc));

    if config.general.enable_cost_tracking {
        svc = tower::util::BoxCloneService::new(CostTrackingLayer.layer(svc));
    }

    if let Some(ref budget_cfg) = config.budget {
        let enforcement = match budget_cfg.enforcement {
            crate::config::EnforcementMode::Soft => Enforcement::Soft,
            crate::config::EnforcementMode::Hard => Enforcement::Hard,
        };
        let tower_budget_cfg = BudgetConfig {
            global_limit: budget_cfg.global_limit,
            model_limits: budget_cfg.model_limits.clone(),
            enforcement,
        };
        let state = Arc::new(BudgetState::new());
        let layer = BudgetLayer::new(tower_budget_cfg, state);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    if config.general.enable_tracing {
        svc = tower::util::BoxCloneService::new(TracingLayer.layer(svc));
    }

    // ~keep HooksLayer sits outside Tracing so every request, including cache hits, emits usage.
    // ~keep `UsageSink` uses RPITIT and is not dyn-compatible, so `MultiUsageSink` erases it.
    if let Some(sink) = usage_sink {
        let multi = Arc::new(MultiUsageSink::from_erased(vec![sink]));
        let layer = HooksLayer::new(vec![]).with_usage_sink(multi);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    svc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProxyConfig;

    fn config_with_one_model() -> ProxyConfig {
        ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "test-model"
provider_model = "openai/gpt-4o"
api_key = "sk-test"
"#,
        )
        .expect("valid TOML")
    }

    fn config_with_two_models() -> ProxyConfig {
        ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "model-a"
provider_model = "openai/gpt-4o"
api_key = "sk-a"

[[models]]
name = "model-b"
provider_model = "anthropic/claude-sonnet-4-20250514"
api_key = "sk-b"
"#,
        )
        .expect("valid TOML")
    }

    #[test]
    fn build_from_empty_config() {
        let config = ProxyConfig::default();
        let pool = ServicePool::from_config(&config, None).expect("empty config should build");
        assert!(pool.services.is_empty());
        assert!(pool.clients.is_empty());
        assert!(!pool.has_any_service());
    }

    #[test]
    fn build_from_config_with_one_model() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        assert_eq!(pool.services.len(), 1);
        assert_eq!(pool.clients.len(), 1);
        assert!(pool.has_any_service());
    }

    #[test]
    fn get_service_for_unknown_model_returns_not_found() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        let result = pool.get_service("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn get_service_for_known_model_succeeds() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        let result = pool.get_service("test-model");
        assert!(result.is_ok());
    }

    #[test]
    fn get_client_for_known_model_succeeds() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        let result = pool.get_client("test-model");
        assert!(result.is_ok());
    }

    #[test]
    fn get_client_for_unknown_model_returns_not_found() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        let result = pool.get_client("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn model_names_returns_correct_list() {
        let config = config_with_two_models();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        let mut names = pool.model_names();
        names.sort();
        assert_eq!(names, vec!["model-a", "model-b"]);
    }

    #[test]
    fn has_any_service_returns_false_for_empty_pool() {
        let config = ProxyConfig::default();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        assert!(!pool.has_any_service());
    }

    #[test]
    fn has_any_service_returns_true_for_nonempty_pool() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config, None).expect("should build");
        assert!(pool.has_any_service());
    }

    #[tokio::test]
    async fn build_with_middleware_config() {
        let config = ProxyConfig::from_toml_str(
            r#"
[general]
enable_cost_tracking = true
enable_tracing = true

[[models]]
name = "gpt"
provider_model = "openai/gpt-4o"
api_key = "sk-test"

[cache]
max_entries = 128
ttl_seconds = 60

[rate_limit]
rpm = 100

[budget]
global_limit = 50.0
enforcement = "soft"

[cooldown]
duration_secs = 30

[health]
interval_secs = 10
"#,
        )
        .expect("valid TOML");

        let pool = ServicePool::from_config(&config, None).expect("should build with middleware");
        assert!(pool.has_any_service());
        assert!(pool.get_service("gpt").is_ok());
    }

    /// Regression test for the "extra deployments are silently dropped" gap:
    /// `ServicePool::from_config` must build successfully — via
    /// [`build_base_service`]'s `Router` path — for a name declared by more
    /// than two entries, not just the two-entry case. `get_client` still
    /// resolves to the first entry, per the documented File/Batch bypass
    /// contract.
    #[test]
    fn deployment_pool_builds_router_across_all_entries() {
        let config = ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "gpt"
provider_model = "openai/gpt-4o"
api_key = "sk-1"

[[models]]
name = "gpt"
provider_model = "anthropic/claude-sonnet-4-5"
api_key = "sk-2"

[[models]]
name = "gpt"
provider_model = "openai/gpt-4o-mini"
api_key = "sk-3"
"#,
        )
        .expect("valid TOML");

        let pool = ServicePool::from_config(&config, None).expect("should build");
        assert_eq!(pool.services.len(), 1);
        assert!(pool.get_service("gpt").is_ok());
        assert!(pool.get_client("gpt").is_ok());
    }

    /// The `Vec` passed to `Router::with_deployment_models` must align 1:1
    /// with the `deployments` vec index Router dispatches on — a misordered
    /// list resolves a classifier verdict to the wrong upstream. Both vecs
    /// are built from the same `entries` iteration in `build_base_service`,
    /// so this proves the extraction itself preserves configured order.
    #[test]
    fn deployment_model_ids_preserves_configured_order() {
        let one = ModelEntry {
            name: "gpt".to_string(),
            provider_model: "openai/gpt-4o".to_string(),
            api_key: None,
            base_url: None,
            timeout_secs: None,
            fallbacks: vec![],
        };
        let two = ModelEntry {
            name: "gpt".to_string(),
            provider_model: "azure/gpt-4o".to_string(),
            api_key: None,
            base_url: None,
            timeout_secs: None,
            fallbacks: vec![],
        };
        let entries: Vec<&ModelEntry> = vec![&one, &two];

        assert_eq!(
            deployment_model_ids(&entries),
            vec!["openai/gpt-4o".to_string(), "azure/gpt-4o".to_string()]
        );
    }
}
