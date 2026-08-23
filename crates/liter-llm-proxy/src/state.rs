use std::sync::Arc;

use arc_swap::ArcSwap;
use liter_llm::guardrail::GuardrailRegistry;
use liter_llm::observability::UsageSinkErased;
use liter_llm::tenant::KeyResolver;

use crate::auth::KeyStore;
use crate::config::ProxyConfig;
use crate::file_store::FileStore;
use crate::secrets::SecretManagerRegistry;
use crate::service_pool::ServicePool;
use crate::shutdown::ShutdownHandle;

/// Shared application state passed to all axum handlers via `State`.
///
/// `config` is an [`ArcSwap`] so that hot-reloads can atomically update the
/// configuration without blocking in-flight requests. Every handler should
/// call `state.config.load()` at request entry to obtain a consistent
/// snapshot for the lifetime of that request.
#[derive(Clone)]
pub struct AppState {
    /// Concrete key store used for master-key checks and model-access control.
    pub key_store: Arc<KeyStore>,
    /// Polymorphic key resolver — the same `KeyStore` exposed via the
    /// [`KeyResolver`] trait so embedding code can swap in custom backends.
    pub key_resolver: Arc<dyn KeyResolver>,
    pub service_pool: Arc<ServicePool>,
    /// The live guardrail set built from `[[guardrails]]` at startup.
    ///
    /// This is the same `Arc` [`ServicePool`] layers into every model's unary
    /// Tower stack (obtain it with [`ServicePool::guardrails`], never by
    /// calling [`crate::guardrail::build_registry`] a second time). The
    /// realtime WebSocket proxy reads it here because it proxies raw frames
    /// and so has no Tower stack to layer.
    ///
    /// Startup-only: `GuardrailLayer` holds its registry by `Arc`, so a config
    /// hot-reload cannot swap this. The watcher warns when a reload changes
    /// `[[guardrails]]` — see [`crate::config::watcher`].
    pub guardrails: Arc<GuardrailRegistry>,
    pub file_store: Arc<FileStore>,
    /// Atomically-swappable proxy configuration.
    ///
    /// Call `state.config.load()` to obtain a reference-counted snapshot.
    /// The snapshot is stable for the lifetime of the returned guard — it will
    /// not change even if a hot-reload fires while the request is in flight.
    pub config: Arc<ArcSwap<ProxyConfig>>,
    /// Secret manager registry for resolving `aws://`, `vault://`, and
    /// `env://` API-key references in model and alias configurations.
    pub secret_registry: Arc<SecretManagerRegistry>,
    /// Optional shutdown handle; present when the server was started via
    /// [`crate::ProxyServer::serve_with_shutdown`].  Health routes read this
    /// to differentiate between `/health/liveness` (200 during drain) and
    /// `/health/readiness` (503 during drain).
    pub shutdown: Option<ShutdownHandle>,
    /// Embedder-supplied usage sink; when `Some`, [`HooksLayer`] is pushed
    /// outermost in the Tower stack so every completed request emits an event.
    pub usage_sink: Option<Arc<dyn UsageSinkErased>>,
}
