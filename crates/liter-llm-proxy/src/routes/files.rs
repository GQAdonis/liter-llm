use axum::Extension;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use liter_llm::client::FileClient;
use liter_llm::types::files::{CreateFileRequest, DeleteResponse, FileListQuery, FileListResponse, FileObject};
use serde::Deserialize;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// Local query struct for list files that does not use `deny_unknown_fields`,
/// allowing callers to pass arbitrary query parameters without rejection.
#[derive(Debug, Default, Deserialize)]
pub struct ListFilesQuery {
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub after: Option<String>,
}

/// POST /v1/files
#[utoipa::path(
    post,
    path = "/v1/files",
    tag = "files",
    request_body(content_type = "application/json", description = "File upload request"),
    responses(
        (status = 200, description = "File object"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_file(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateFileRequest>,
) -> Result<Json<FileObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "create_file")?;
    let client = state.service_pool.first_client()?;
    let result = client.create_file(req).await?;
    Ok(Json(result))
}

/// GET /v1/files
#[utoipa::path(
    get,
    path = "/v1/files",
    tag = "files",
    params(
        ("purpose" = Option<String>, Query, description = "Filter by purpose"),
        ("limit" = Option<u32>, Query, description = "Maximum number of results"),
        ("after" = Option<String>, Query, description = "Cursor for pagination"),
    ),
    responses(
        (status = 200, description = "List of file objects"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn list_files(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Query(params): Query<ListFilesQuery>,
) -> Result<Json<FileListResponse>, ProxyError> {
    crate::auth::require_master(&key_ctx, "list_files")?;
    let client = state.service_pool.first_client()?;
    let query = if params.purpose.is_some() || params.limit.is_some() || params.after.is_some() {
        Some(FileListQuery {
            purpose: params.purpose,
            limit: params.limit,
            after: params.after,
        })
    } else {
        None
    };
    let result = client.list_files(query).await?;
    Ok(Json(result))
}

/// GET /v1/files/{file_id}
#[utoipa::path(
    get,
    path = "/v1/files/{file_id}",
    tag = "files",
    params(("file_id" = String, Path, description = "File identifier")),
    responses(
        (status = 200, description = "File object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn retrieve_file(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(file_id): Path<String>,
) -> Result<Json<FileObject>, ProxyError> {
    crate::auth::require_master(&key_ctx, "retrieve_file")?;
    let client = state.service_pool.first_client()?;
    let result = client.retrieve_file(&file_id).await?;
    Ok(Json(result))
}

/// DELETE /v1/files/{file_id}
#[utoipa::path(
    delete,
    path = "/v1/files/{file_id}",
    tag = "files",
    params(("file_id" = String, Path, description = "File identifier")),
    responses(
        (status = 200, description = "Deletion confirmation"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn delete_file(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(file_id): Path<String>,
) -> Result<Json<DeleteResponse>, ProxyError> {
    crate::auth::require_master(&key_ctx, "delete_file")?;
    let client = state.service_pool.first_client()?;
    let result = client.delete_file(&file_id).await?;
    Ok(Json(result))
}

/// GET /v1/files/{file_id}/content
#[utoipa::path(
    get,
    path = "/v1/files/{file_id}/content",
    tag = "files",
    params(("file_id" = String, Path, description = "File identifier")),
    responses(
        (status = 200, description = "File content bytes", content_type = "application/octet-stream"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn file_content(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Path(file_id): Path<String>,
) -> Result<Response, ProxyError> {
    crate::auth::require_master(&key_ctx, "file_content")?;
    let client = state.service_pool.first_client()?;
    let bytes = client.file_content(&file_id).await?;
    Ok(([(CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arc_swap::ArcSwap;
    use liter_llm::types::files::FilePurpose;

    use super::*;
    use crate::auth::KeyStore;
    use crate::config::{FileStorageConfig, ProxyConfig, VirtualKeyConfig};
    use crate::file_store::FileStore;
    use crate::secrets::{EnvVarSecretManager, SecretManager};
    use crate::service_pool::ServicePool;

    /// Build a minimal `AppState` with one configured model and no live
    /// network access performed at construction time (`DefaultClient::new`
    /// only builds an HTTP client value).
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
            crate::secrets::SecretManagerRegistry::builder()
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
            description: None,
            models: vec![],
            rpm: None,
            tpm: None,
            budget_limit: None,
            provider_credentials: vec![],
        };
        KeyContext::from_config(&cfg)
    }

    /// Regression test for the cross-tenant leak: before the `require_master`
    /// guard, `create_file` discarded `key_ctx` entirely and any virtual key
    /// could create/read/delete files in the shared upstream account.
    #[tokio::test]
    async fn create_file_rejects_non_master_key() {
        let state = test_state();
        let req = CreateFileRequest {
            file: "aGVsbG8=".to_string(),
            purpose: FilePurpose::Assistants,
            filename: Some("test.txt".to_string()),
        };

        let result = create_file(State(state), Extension(virtual_key_ctx()), Json(req)).await;
        assert!(
            result.is_err(),
            "non-master virtual key must not be able to create files"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }

    #[tokio::test]
    async fn list_files_rejects_non_master_key() {
        let state = test_state();
        let result = list_files(
            State(state),
            Extension(virtual_key_ctx()),
            Query(ListFilesQuery::default()),
        )
        .await;
        assert!(
            result.is_err(),
            "non-master virtual key must not list files belonging to other tenants"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }

    #[tokio::test]
    async fn retrieve_file_rejects_non_master_key() {
        let state = test_state();
        let result = retrieve_file(State(state), Extension(virtual_key_ctx()), Path("file-abc".to_string())).await;
        assert!(
            result.is_err(),
            "non-master virtual key must not retrieve another tenant's file"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }

    #[tokio::test]
    async fn delete_file_rejects_non_master_key() {
        let state = test_state();
        let result = delete_file(State(state), Extension(virtual_key_ctx()), Path("file-abc".to_string())).await;
        assert!(
            result.is_err(),
            "non-master virtual key must not delete another tenant's file"
        );
        assert_eq!(result.unwrap_err().error_type(), "Forbidden");
    }
}
