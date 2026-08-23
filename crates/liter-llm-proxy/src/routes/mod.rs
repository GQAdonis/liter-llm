pub mod audio;
pub mod batches;
pub mod chat;
pub mod embeddings;
pub mod files;
pub mod health;
pub mod images;
pub mod models;
pub mod moderations;
pub mod ocr;
pub mod realtime;
pub mod rerank;
pub mod responses;
pub mod search;

use std::time::Duration;

use axum::Router;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::middleware;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::TraceLayer;

use tower::Service;

use liter_llm::error::LiterLlmError;
use liter_llm::tower::types::{LlmRequest, LlmResponse};

use crate::auth;
use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// Bound how long a unary handler may take to produce a `Response`.
///
/// `next.run(request)` resolves the moment the inner handler *returns* a
/// `Response` — for an SSE/streaming chat completion that happens as soon as
/// the upstream connection responds and the stream begins, well before the
/// body finishes draining (the body is polled separately by hyper after this
/// future resolves). So `tokio::time::timeout` wrapped around it bounds a
/// hung/slow upstream that never responds at all; it can never cut short an
/// already-flowing stream. Applied only to the unary route group in
/// `build_router` — never to `/v1/realtime`, whose WebSocket session must
/// never be time-boxed (see that route's module docs).
///
/// Reuses [`LiterLlmError::Timeout`]'s existing 504 mapping (via
/// `From<LiterLlmError> for ProxyError`) rather than a bespoke error variant,
/// so a server-side request timeout reports identically to an upstream
/// client timeout.
async fn request_timeout_middleware(State(timeout): State<Duration>, request: Request, next: Next) -> Response {
    match tokio::time::timeout(timeout, next.run(request)).await {
        Ok(response) => response,
        Err(_) => ProxyError::from(LiterLlmError::Timeout).into_response(),
    }
}

/// Check model access for the authenticated key, attach the tenant identifier,
/// and dispatch the request through the Tower service stack.
///
/// `tenant_id` is propagated via [`LlmRequest::with_tenant_id`] so that every
/// Tower layer downstream — [`BudgetLedger::Tenant`], [`TenantScopedStrategy`],
/// and [`UsageEvent`] — receives the correct tenant dimension without each
/// handler needing to wire it independently.
pub(crate) async fn dispatch(
    state: &AppState,
    key_ctx: &KeyContext,
    model: &str,
    request: LlmRequest,
) -> Result<LlmResponse, ProxyError> {
    if !key_ctx.can_access_model(model) {
        return Err(ProxyError::forbidden(format!(
            "key '{}' is not allowed to access model '{model}'",
            key_ctx.redacted_id()
        )));
    }
    let request = request.with_tenant_id(key_ctx.tenant_id.clone());
    let mut svc = state.service_pool.get_service(model)?;
    Ok(svc.call(request).await?)
}

/// Build the full axum router with all routes, middleware, and shared state.
pub fn build_router(state: AppState) -> Router {
    // ~keep Router-build config is startup-only; handlers must load config per request.
    let cfg_snapshot = state.config.load();

    let request_timeout = Duration::from_secs(cfg_snapshot.server.request_timeout_secs);

    let unary_v1_routes = Router::new()
        .route("/v1/chat/completions", post(chat::chat_completions))
        .route("/v1/embeddings", post(embeddings::create_embedding))
        .route("/v1/models", get(models::list_models))
        .route("/v1/images/generations", post(images::create_image))
        .route("/v1/audio/speech", post(audio::create_speech))
        .route("/v1/audio/transcriptions", post(audio::create_transcription))
        .route("/v1/moderations", post(moderations::create_moderation))
        .route("/v1/rerank", post(rerank::rerank))
        .route("/v1/search", post(search::search))
        .route("/v1/ocr", post(ocr::ocr))
        .route("/v1/files", post(files::create_file).get(files::list_files))
        .route(
            "/v1/files/{file_id}",
            get(files::retrieve_file).delete(files::delete_file),
        )
        .route("/v1/files/{file_id}/content", get(files::file_content))
        .route("/v1/batches", post(batches::create_batch).get(batches::list_batches))
        .route("/v1/batches/{batch_id}", get(batches::retrieve_batch))
        .route("/v1/batches/{batch_id}/cancel", post(batches::cancel_batch))
        .route("/v1/responses", post(responses::create_response))
        .route("/v1/responses/{response_id}", get(responses::retrieve_response))
        .route("/v1/responses/{response_id}/cancel", post(responses::cancel_response))
        .layer(middleware::from_fn_with_state(
            request_timeout,
            request_timeout_middleware,
        ));

    // ~keep `/v1/realtime` is a long-lived WebSocket upgrade, split out of
    // ~keep `unary_v1_routes` so the request timeout above never applies to
    // ~keep it — see the module docs on `realtime_websocket` for the
    // ~keep rpm/budget enforcement that route gets instead.
    let realtime_routes = Router::new().route("/v1/realtime", get(realtime::realtime_websocket));

    let v1_routes = Router::new()
        .merge(unary_v1_routes)
        .merge(realtime_routes)
        .layer(middleware::from_fn_with_state(state.clone(), auth::validate_api_key));

    let health_routes = Router::new()
        .route("/health", get(health::health))
        .route("/health/liveness", get(health::liveness))
        .route("/health/readiness", get(health::readiness))
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/openapi.json", get(crate::openapi::openapi_schema));

    // ~keep Wildcard CORS must not expose Authorization or it permits credentialed requests.
    let cors_layer: Option<CorsLayer> = if cfg_snapshot.server.cors_origins.is_empty() {
        None
    } else if cfg_snapshot.server.cors_origins.iter().any(|o| o == "*") {
        Some(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                // ~keep Deliberately exclude Authorization for wildcard origins.
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::ACCEPT]),
        )
    } else {
        let origins: Vec<HeaderValue> = cfg_snapshot
            .server
            .cors_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        Some(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list(origins))
                .allow_methods(Any)
                .allow_headers(Any),
        )
    };

    let mut router = Router::new()
        .merge(v1_routes)
        .merge(health_routes)
        .layer(SetSensitiveHeadersLayer::new([AUTHORIZATION]))
        .layer(DefaultBodyLimit::max(cfg_snapshot.server.body_limit_bytes))
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    if let Some(layer) = cors_layer {
        router = router.layer(layer);
    }

    router
}

#[cfg(test)]
mod tests {
    use axum::body::{Body, Bytes};
    use axum::http::{Request as HttpRequest, StatusCode};
    use axum::routing::get;
    use futures_util::stream;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    async fn slow_handler() -> &'static str {
        tokio::time::sleep(Duration::from_millis(50)).await;
        "ok"
    }

    async fn fast_handler() -> &'static str {
        "ok"
    }

    /// Regression test for the "`request_timeout_secs` is parsed but never
    /// applied" gap: a unary handler that outlives the configured timeout
    /// must be cut off with a 504, not left to hang indefinitely.
    #[tokio::test]
    async fn request_timeout_middleware_returns_504_when_handler_is_slow() {
        let app = Router::new()
            .route("/slow", get(slow_handler))
            .layer(middleware::from_fn_with_state(
                Duration::from_millis(5),
                request_timeout_middleware,
            ));

        let response = app
            .oneshot(HttpRequest::builder().uri("/slow").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
    }

    /// A handler that returns comfortably inside the timeout must be
    /// unaffected.
    #[tokio::test]
    async fn request_timeout_middleware_allows_fast_handlers() {
        let app = Router::new()
            .route("/fast", get(fast_handler))
            .layer(middleware::from_fn_with_state(
                Duration::from_secs(5),
                request_timeout_middleware,
            ));

        let response = app
            .oneshot(HttpRequest::builder().uri("/fast").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    /// The core regression this middleware's placement exists to prevent: a
    /// streaming body whose TOTAL drain time exceeds `request_timeout` must
    /// still complete in full, because the timeout only bounds how long the
    /// handler takes to *return* a `Response` — not how long the body then
    /// takes to stream. A request timeout that killed a healthy in-progress
    /// stream would be worse than no timeout at all.
    #[tokio::test]
    async fn request_timeout_middleware_does_not_cut_off_a_slow_streaming_body() {
        async fn streaming_handler() -> Response {
            let body_stream = stream::unfold(0u8, |i| async move {
                if i >= 3 {
                    return None;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
                Some((Ok::<_, std::io::Error>(Bytes::from_static(b"chunk")), i + 1))
            });
            Response::new(Body::from_stream(body_stream))
        }

        let app = Router::new()
            .route("/stream", get(streaming_handler))
            .layer(middleware::from_fn_with_state(
                Duration::from_millis(10),
                request_timeout_middleware,
            ));

        let response = app
            .oneshot(HttpRequest::builder().uri("/stream").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "the handler returns immediately, well within the 10ms timeout"
        );

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(
            &bytes[..],
            b"chunkchunkchunk".as_slice(),
            "the ~60ms stream must drain in full even though request_timeout=10ms"
        );
    }
}
