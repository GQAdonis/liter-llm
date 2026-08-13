use axum::Extension;
use axum::Json;
use axum::extract::{Path, State};
use liter_llm::client::ResponseClient;
use liter_llm::types::responses::{CreateResponseRequest, ResponseObject};

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/responses
#[utoipa::path(
    post,
    path = "/v1/responses",
    tag = "responses",
    request_body(content_type = "application/json", description = "Create response request"),
    responses(
        (status = 200, description = "Response object"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_response(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateResponseRequest>,
) -> Result<Json<ResponseObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "create_response")?;
    let client = state.service_pool.first_client()?;
    let result = client.create_response(req).await?;
    Ok(Json(result))
}

/// GET /v1/responses/{response_id}
#[utoipa::path(
    get,
    path = "/v1/responses/{response_id}",
    tag = "responses",
    params(("response_id" = String, Path, description = "Response identifier")),
    responses(
        (status = 200, description = "Response object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn retrieve_response(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(response_id): Path<String>,
) -> Result<Json<ResponseObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "retrieve_response")?;
    let client = state.service_pool.first_client()?;
    let result = client.retrieve_response(&response_id).await?;
    Ok(Json(result))
}

/// POST /v1/responses/{response_id}/cancel
#[utoipa::path(
    post,
    path = "/v1/responses/{response_id}/cancel",
    tag = "responses",
    params(("response_id" = String, Path, description = "Response identifier")),
    responses(
        (status = 200, description = "Cancelled response object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn cancel_response(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(response_id): Path<String>,
) -> Result<Json<ResponseObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "cancel_response")?;
    let client = state.service_pool.first_client()?;
    let result = client.cancel_response(&response_id).await?;
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

    /// Regression test: before the `require_master` guard, `create_response`
    /// used `require_model_access` — the wrong guard for a resource-lifecycle
    /// endpoint bypassing model routing entirely (see issue #70) — so any
    /// virtual key could create a response in the shared upstream account.
    #[tokio::test]
    async fn create_response_rejects_non_master_key() {
        let state = test_state();
        let req = CreateResponseRequest {
            model: "test-model".to_string(),
            input: serde_json::Value::String("hello".to_string()),
            instructions: None,
            tools: None,
            temperature: None,
            max_output_tokens: None,
            metadata: None,
            stream: None,
        };

        let result = create_response(State(state), Extension(virtual_key_ctx()), Json(req)).await;
        assert!(
            result.is_err(),
            "non-master virtual key must not be able to create responses"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }

    #[tokio::test]
    async fn retrieve_response_rejects_non_master_key() {
        let state = test_state();
        let result = retrieve_response(State(state), Extension(virtual_key_ctx()), Path("resp-abc".to_string())).await;
        assert!(
            result.is_err(),
            "non-master virtual key must not retrieve another tenant's response"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }
}
