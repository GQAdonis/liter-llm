pub mod key_store;

pub use key_store::{KeyContext, KeyStore, MASTER_TENANT_ID};

use axum::extract::{Request, State};
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;
use liter_llm::tenant::KeyResolverError;

use crate::error::ProxyError;
use crate::state::AppState;

/// Axum middleware that validates the `Authorization: Bearer <token>` header
/// against the configured master key and virtual key store.
///
/// On success the resolved [`KeyContext`] — including a populated `tenant_id`
/// — is inserted into request extensions so downstream handlers can inspect
/// model-access permissions and attach the tenant to outbound [`liter_llm::tower::types::LlmRequest`]s.
pub async fn validate_api_key(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ProxyError> {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| ProxyError::authentication("Missing or invalid Authorization header"))?;

    if state.key_store.is_master_key(token) {
        request.extensions_mut().insert(KeyContext::master());
        return Ok(next.run(request).await);
    }

    let token_owned = token.to_owned();
    let resolved = state.key_resolver.resolve(token_owned).await.map_err(|e| match e {
        KeyResolverError::NotFound | KeyResolverError::Inactive => ProxyError::authentication("Invalid API key"),
        KeyResolverError::Backend(msg) => ProxyError::internal(format!("key resolver backend error: {msg}")),
    })?;

    let ctx = KeyContext::from_resolved(token, &resolved);
    request.extensions_mut().insert(ctx);
    Ok(next.run(request).await)
}

/// Require the master key for resource-lifecycle endpoints (files, batches,
/// responses).
///
/// These endpoints bypass model routing via `ServicePool::first_client()` and
/// talk directly to a single upstream provider account: file/batch/response
/// IDs are opaque identifiers owned by that account, not by this proxy, so
/// there is no per-tenant record to scope a virtual key's access to. Mirrors
/// `LiterLlmMcp::require_master` in `mcp/tools.rs`, which restricts the
/// equivalent MCP tools for the same reason — allowing any virtual key
/// through would let one tenant list, read, or delete another tenant's
/// files/batches/responses.
pub fn require_master(key_ctx: &KeyContext, endpoint: &str) -> Result<(), ProxyError> {
    if key_ctx.is_master {
        return Ok(());
    }
    Err(ProxyError::forbidden(format!(
        "endpoint '{endpoint}' requires master-key access; key '{}' is restricted",
        key_ctx.redacted_id()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::VirtualKeyConfig;

    fn restricted_ctx(key_id: &str) -> KeyContext {
        let cfg = VirtualKeyConfig {
            key: key_id.to_string(),
            description: None,
            models: vec![],
            rpm: None,
            tpm: None,
            budget_limit: None,
            provider_credentials: vec![],
        };
        KeyContext::from_config(&cfg)
    }

    #[test]
    fn require_master_allows_master_key() {
        let ctx = KeyContext::master();
        assert!(require_master(&ctx, "create_file").is_ok());
    }

    #[test]
    fn require_master_rejects_virtual_key() {
        let ctx = restricted_ctx("vk-tenant-a");
        let result = require_master(&ctx, "create_file");
        assert!(
            result.is_err(),
            "virtual key must be rejected for master-only endpoints"
        );
        let err = result.unwrap_err();
        assert_eq!(err.error_type(), "Forbidden");
        assert!(err.to_string().contains("create_file"));
        // ~keep The 403 body is returned to the caller and re-ingested by every log and
        // ~keep error tracker on the response path, so it must carry the stable redacted
        // ~keep correlation id, never the live key. This assertion previously required the
        // ~keep opposite — it pinned the leak in place.
        assert!(
            !err.to_string().contains("vk-tenant-a"),
            "the live virtual key must never reach a client-visible error body"
        );
        assert!(
            err.to_string().contains(&restricted_ctx("vk-tenant-a").redacted_id()),
            "the error must still identify the key by its redacted correlation id"
        );
    }
}
