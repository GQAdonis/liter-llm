use axum::Extension;
use axum::Json;
use axum::extract::{Path, Query, State};
use liter_llm::client::BatchClient;
use liter_llm::types::batch::{BatchListQuery, BatchListResponse, BatchObject, CreateBatchRequest};
use serde::Deserialize;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// Local query struct for list batches that does not use `deny_unknown_fields`,
/// allowing callers to pass arbitrary query parameters without rejection.
#[derive(Debug, Default, Deserialize)]
pub struct ListBatchesQuery {
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub after: Option<String>,
}

/// POST /v1/batches
#[utoipa::path(
    post,
    path = "/v1/batches",
    tag = "batches",
    request_body(content_type = "application/json", description = "Create batch request"),
    responses(
        (status = 200, description = "Batch object"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_batch(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateBatchRequest>,
) -> Result<Json<BatchObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "create_batch")?;
    let client = state.service_pool.first_client()?;
    let result = client.create_batch(req).await?;
    Ok(Json(result))
}

/// GET /v1/batches
#[utoipa::path(
    get,
    path = "/v1/batches",
    tag = "batches",
    params(
        ("limit" = Option<u32>, Query, description = "Maximum number of results"),
        ("after" = Option<String>, Query, description = "Cursor for pagination"),
    ),
    responses(
        (status = 200, description = "List of batch objects"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn list_batches(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Query(params): Query<ListBatchesQuery>,
) -> Result<Json<BatchListResponse>, ProxyError> {
    crate::auth::require_master(&key_ctx, "list_batches")?;
    let client = state.service_pool.first_client()?;
    let query = if params.limit.is_some() || params.after.is_some() {
        Some(BatchListQuery {
            limit: params.limit,
            after: params.after,
        })
    } else {
        None
    };
    let result = client.list_batches(query).await?;
    Ok(Json(result))
}

/// GET /v1/batches/{batch_id}
#[utoipa::path(
    get,
    path = "/v1/batches/{batch_id}",
    tag = "batches",
    params(("batch_id" = String, Path, description = "Batch identifier")),
    responses(
        (status = 200, description = "Batch object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn retrieve_batch(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "retrieve_batch")?;
    let client = state.service_pool.first_client()?;
    let result = client.retrieve_batch(&batch_id).await?;
    Ok(Json(result))
}

/// POST /v1/batches/{batch_id}/cancel
#[utoipa::path(
    post,
    path = "/v1/batches/{batch_id}/cancel",
    tag = "batches",
    params(("batch_id" = String, Path, description = "Batch identifier")),
    responses(
        (status = 200, description = "Cancelled batch object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn cancel_batch(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "cancel_batch")?;
    let client = state.service_pool.first_client()?;
    let result = client.cancel_batch(&batch_id).await?;
    Ok(Json(result))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arc_swap::ArcSwap;

    use super::*;
    use crate::auth::KeyStore;
    use crate::config::{FileStorageConfig, ProxyConfig, VirtualKeyConfig};
    use crate::file_store::FileStore;
    use crate::secrets::{EnvVarSecretManager, SecretManager, SecretManagerRegistry};
    use crate::service_pool::ServicePool;

    /// Mirrors `routes::files::tests::test_state` — see that module for why
    /// no network access happens at construction time.
    fn test_state() -> AppState {
        let config = ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "test-model"
provider_model = "openai/gpt-4o"
api_key = "sk-test"
"#,
        )
        .expect("valid TOML");

        let service_pool = Arc::new(ServicePool::from_config(&config, None).expect("service pool"));
        let file_store = Arc::new(FileStore::from_config(&FileStorageConfig::default()).expect("file store"));
        let key_store = Arc::new(KeyStore::from_config(None, &[]));
        let secret_registry = Arc::new(
            SecretManagerRegistry::builder()
                .default_backend(Arc::new(EnvVarSecretManager::new()) as Arc<dyn SecretManager>)
                .build(),
        );

        AppState {
            key_resolver: key_store.clone() as Arc<dyn liter_llm::tenant::KeyResolver>,
            key_store,
            guardrails: service_pool.guardrails(),
            service_pool,
            file_store,
            config: Arc::new(ArcSwap::from(Arc::new(config))),
            secret_registry,
            shutdown: None,
            usage_sink: None,
        }
    }

    fn virtual_key_ctx() -> KeyContext {
        let cfg = VirtualKeyConfig {
            key: "vk-tenant-a".to_string(),
            tenant_id: None,
            description: None,
            models: vec![],
            rpm: None,
            tpm: None,
            budget_limit: None,
            provider_credentials: vec![],
        };
        KeyContext::from_config(&cfg)
    }

    /// Regression test: before the `require_master` guard, `create_batch`
    /// discarded `key_ctx` and any virtual key could create a batch in the
    /// shared upstream account.
    #[tokio::test]
    async fn create_batch_rejects_non_master_key() {
        let state = test_state();
        let req = CreateBatchRequest {
            input_file_id: "file-abc".to_string(),
            endpoint: "/v1/chat/completions".to_string(),
            completion_window: "24h".to_string(),
            metadata: None,
        };

        let result = create_batch(State(state), Extension(virtual_key_ctx()), Json(req)).await;
        assert!(
            result.is_err(),
            "non-master virtual key must not be able to create batches"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }

    #[tokio::test]
    async fn retrieve_batch_rejects_non_master_key() {
        let state = test_state();
        let result = retrieve_batch(
            State(state),
            Extension(virtual_key_ctx()),
            Path("batch-abc".to_string()),
        )
        .await;
        assert!(
            result.is_err(),
            "non-master virtual key must not retrieve another tenant's batch"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }
}
