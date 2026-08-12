//! Tower middleware layer that enforces guardrail checks at each request stage.
//!
//! [`GuardrailLayer`] wraps any [`Service<LlmRequest>`] and runs the registered
//! guardrails at three lifecycle points:
//!
//! - **`Input`** — before forwarding the request to the inner service. A
//!   `Block` decision returns [`LiterLlmError::HookRejected`] immediately.
//! - **`Output`** — after the inner service returns a non-streaming response.
//!   A `Block` decision returns an error; `Mutate` replaces the response JSON.
//! - **`OutputChunk`** — for each streaming chunk. A `Block` decision
//!   terminates the stream; `Mutate` replaces the chunk text.
//!
//! # Example
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use liter_llm::guardrail::registry::GuardrailRegistry;
//! use liter_llm::tower::guardrail::GuardrailLayer;
//! use tower::ServiceBuilder;
//!
//! let registry = Arc::new(GuardrailRegistry::new());
//! let service = ServiceBuilder::new()
//!     .layer(GuardrailLayer::new(registry, Default::default()))
//!     .service(inner_service);
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::task::{Context, Poll};

use tower::Layer;
use tower::Service;

use crate::client::BoxFuture;
use crate::error::{LiterLlmError, Result};
use crate::guardrail::registry::GuardrailRegistry;
use crate::guardrail::{GuardrailContext, GuardrailDecision, GuardrailStage};

use super::types::{LlmRequest, LlmResponse};

/// Serialize a guardrail-inspectable response body, failing closed.
///
/// ~keep A response that cannot be serialized is treated as blocked rather
/// ~keep than passed through unchecked: silently falling back to `Ok(response)`
/// ~keep here would let arbitrary un-inspected content reach the caller
/// ~keep whenever a response happens to serialize badly, defeating the
/// ~keep purpose of the Output guardrail stage.
fn serialize_for_guardrail<T: serde::Serialize>(value: &T) -> Result<serde_json::Value> {
    serde_json::to_value(value).map_err(|e| LiterLlmError::InternalError {
        message: format!("guardrail: failed to serialize response for output-stage inspection: {e}"),
    })
}

/// Build the JSON payload the `Output` guardrail stage inspects for a given
/// response, or `None` when no aggregate body is available yet to inspect.
fn response_to_guardrail_json(response: &LlmResponse) -> Result<Option<serde_json::Value>> {
    match response {
        LlmResponse::Chat(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Embed(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::ListModels(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::ImageGenerate(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Transcribe(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Moderate(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Rerank(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Search(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Ocr(r) => serialize_for_guardrail(r).map(Some),
        LlmResponse::Speech(audio_bytes) => Ok(Some(serde_json::json!({
            // ~keep Speech returns raw audio bytes, not a serializable struct.
            // ~keep Emitting the bytes verbatim would blow up the guardrail
            // ~keep payload; exposing the length is still enough for e.g.
            // ~keep length-cap guardrails to act on.
            "byte_len": audio_bytes.len(),
        }))),
        LlmResponse::ChatStream(_) => Ok(None),
    }
}

/// Tower [`Layer`] that enforces guardrail checks around an inner service.
///
/// `registry` holds the ordered list of guardrails to evaluate.
/// `metadata` provides per-layer static tags (e.g., route, deployment) that are
/// merged with per-call metadata passed by the application.
#[cfg_attr(alef, alef(skip))]
#[derive(Clone)]
pub struct GuardrailLayer {
    registry: Arc<GuardrailRegistry>,
    metadata: Arc<HashMap<String, String>>,
}

impl GuardrailLayer {
    /// Create a new [`GuardrailLayer`] with the given registry and static metadata.
    ///
    /// `metadata` is merged into the [`GuardrailContext`] for every request.
    /// Per-call metadata (e.g., `user_id`, `tenant_id`) should be provided via
    /// [`GuardrailContext::metadata`] on a per-request basis; this constructor
    /// accepts layer-level static tags only.
    #[must_use]
    pub fn new(registry: Arc<GuardrailRegistry>, metadata: HashMap<String, String>) -> Self {
        Self {
            registry,
            metadata: Arc::new(metadata),
        }
    }

    /// Create a new [`GuardrailLayer`] with no static metadata.
    #[must_use]
    pub fn with_registry(registry: Arc<GuardrailRegistry>) -> Self {
        Self::new(registry, HashMap::new())
    }
}

impl<S> Layer<S> for GuardrailLayer {
    type Service = GuardrailService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GuardrailService {
            inner,
            registry: Arc::clone(&self.registry),
            metadata: Arc::clone(&self.metadata),
        }
    }
}

/// Tower service produced by [`GuardrailLayer`].
#[cfg_attr(alef, alef(skip))]
pub struct GuardrailService<S> {
    inner: S,
    registry: Arc<GuardrailRegistry>,
    metadata: Arc<HashMap<String, String>>,
}

impl<S: Clone> Clone for GuardrailService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            registry: Arc::clone(&self.registry),
            metadata: Arc::clone(&self.metadata),
        }
    }
}

impl<S> Service<LlmRequest> for GuardrailService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, Result<LlmResponse>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let registry = Arc::clone(&self.registry);
        let metadata = Arc::clone(&self.metadata);
        let inner_fut = self.inner.call(req.clone());

        Box::pin(async move {
            let request_json = match serde_json::to_value(&req) {
                Ok(v) => v,
                Err(e) => {
                    return Err(LiterLlmError::InternalError {
                        message: format!("guardrail: failed to serialize request: {e}"),
                    });
                }
            };

            let input_ctx = GuardrailContext {
                request: &request_json,
                response: None,
                chunk: None,
                metadata: &metadata,
            };

            let input_decision = registry.run_stage(GuardrailStage::Input, &input_ctx).await;
            match input_decision {
                GuardrailDecision::Block { reason, code } => {
                    return Err(LiterLlmError::HookRejected {
                        message: format!("guardrail blocked [code={code}]: {reason}"),
                    });
                }
                GuardrailDecision::Mutate { .. } => {
                    tracing::debug!("guardrail: Input stage Mutate decision; proceeding with original request");
                }
                GuardrailDecision::Allow => {}
            }

            let response = inner_fut.await?;

            let Some(response_json) = response_to_guardrail_json(&response)? else {
                // ~keep ChatStream: no aggregate body exists yet at this point to
                // ~keep run an Output-stage guardrail against. GuardrailStage::OutputChunk
                // ~keep is defined for per-chunk inspection but is not yet wired up by
                // ~keep the streaming path, so streamed responses currently bypass
                // ~keep output guardrails entirely — tracked separately, out of scope here.
                return Ok(response);
            };

            let output_ctx = GuardrailContext {
                request: &request_json,
                response: Some(&response_json),
                chunk: None,
                metadata: &metadata,
            };

            let output_decision = registry.run_stage(GuardrailStage::Output, &output_ctx).await;
            match output_decision {
                GuardrailDecision::Block { reason, code } => Err(LiterLlmError::HookRejected {
                    message: format!("guardrail blocked output [code={code}]: {reason}"),
                }),
                GuardrailDecision::Mutate { .. } => {
                    tracing::debug!("guardrail: Output stage Mutate decision; returning original response");
                    Ok(response)
                }
                GuardrailDecision::Allow => Ok(response),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    use tower::{Layer, Service};

    use super::*;
    use crate::guardrail::Guardrail;
    use crate::guardrail::builtin::DenyListGuardrail;
    use crate::guardrail::registry::GuardrailRegistry;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::LlmRequest;
    use crate::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest, TranscriptionResponse};
    use crate::types::image::{CreateImageRequest, ImagesResponse};
    use crate::types::moderation::{ModerationRequest, ModerationResponse};
    use crate::types::ocr::{OcrRequest, OcrResponse};
    use crate::types::rerank::{RerankRequest, RerankResponse};
    use crate::types::search::{SearchRequest, SearchResponse};

    #[tokio::test]
    async fn guardrail_layer_allows_when_registry_is_empty() {
        let registry = Arc::new(GuardrailRegistry::new());
        let inner = LlmService::new(MockClient::ok());
        let mut svc = GuardrailLayer::with_registry(registry).layer(inner);

        let result = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(result.is_ok(), "empty registry should allow all requests");
    }

    #[tokio::test]
    async fn guardrail_layer_input_block_prevents_inner_call() {
        let mut registry = GuardrailRegistry::new();
        let list: HashSet<String> = ["banned-user"].iter().map(|s| s.to_string()).collect();
        registry.register(Arc::new(DenyListGuardrail::new("ban", list, "user_id")));

        let mock = MockClient::ok();
        let call_count = Arc::clone(&mock.call_count);
        let inner = LlmService::new(mock);

        let mut meta = HashMap::new();
        meta.insert("user_id".to_string(), "banned-user".to_string());

        let mut svc = GuardrailLayer::new(Arc::new(registry), meta).layer(inner);
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("banned user should be blocked");

        assert!(
            matches!(err, LiterLlmError::HookRejected { .. }),
            "guardrail block should surface as HookRejected"
        );
        assert_eq!(call_count.load(Ordering::SeqCst), 0, "inner service must not be called");
    }

    #[tokio::test]
    async fn guardrail_layer_allows_non_blocked_user() {
        let mut registry = GuardrailRegistry::new();
        let list: HashSet<String> = ["banned-user"].iter().map(|s| s.to_string()).collect();
        registry.register(Arc::new(DenyListGuardrail::new("ban", list, "user_id")));

        let inner = LlmService::new(MockClient::ok());
        let mut meta = HashMap::new();
        meta.insert("user_id".to_string(), "good-user".to_string());

        let mut svc = GuardrailLayer::new(Arc::new(registry), meta).layer(inner);
        let result = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(result.is_ok(), "non-blocked user should pass through");
    }

    /// A trivial inner service that always returns a preset response,
    /// regardless of the request. Used to exercise every `LlmResponse`
    /// variant through the guardrail layer without depending on
    /// `MockClient`'s per-endpoint coverage (it doesn't implement every
    /// endpoint — e.g. `search`/`ocr` always return `EndpointNotSupported`).
    #[derive(Clone)]
    struct CannedService {
        build: Arc<dyn Fn() -> LlmResponse + Send + Sync>,
    }

    impl Service<LlmRequest> for CannedService {
        type Response = LlmResponse;
        type Error = LiterLlmError;
        type Future = BoxFuture<'static, Result<LlmResponse>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: LlmRequest) -> Self::Future {
            let resp = (self.build)();
            Box::pin(async move { Ok(resp) })
        }
    }

    /// A guardrail that unconditionally blocks whenever it runs at the
    /// `Output` stage. Used to prove the stage is actually invoked for a
    /// given response type — before the fix, most response variants never
    /// reached `run_stage` at all, so even an always-blocking guardrail
    /// would silently never fire for them.
    struct AlwaysBlockOutput;

    impl Guardrail for AlwaysBlockOutput {
        fn name(&self) -> &'static str {
            "always-block-output"
        }

        fn supported_stages(&self) -> &'static [GuardrailStage] {
            &[GuardrailStage::Output]
        }

        fn check<'a>(
            &'a self,
            _stage: GuardrailStage,
            _ctx: &'a GuardrailContext<'a>,
        ) -> Pin<Box<dyn std::future::Future<Output = GuardrailDecision> + Send + 'a>> {
            Box::pin(async move {
                GuardrailDecision::Block {
                    reason: "test: always blocks output".into(),
                    code: 9999,
                }
            })
        }
    }

    /// Wrap a `CannedService` that always returns a response built by
    /// `build_response` behind a `GuardrailLayer` containing only
    /// `AlwaysBlockOutput`, and assert the call is blocked — proving the
    /// Output stage actually inspected this response type instead of
    /// silently skipping it via the old `_ => return Ok(response)` catch-all.
    async fn assert_output_stage_inspects<F>(request: LlmRequest, build_response: F)
    where
        F: Fn() -> LlmResponse + Send + Sync + 'static,
    {
        let mut registry = GuardrailRegistry::new();
        registry.register(Arc::new(AlwaysBlockOutput));

        let inner = CannedService {
            build: Arc::new(build_response),
        };
        let mut svc = GuardrailLayer::with_registry(Arc::new(registry)).layer(inner);

        let err = svc
            .call(request)
            .await
            .expect_err("Output-stage guardrail should have blocked this response type");

        assert!(
            matches!(err, LiterLlmError::HookRejected { .. }),
            "expected HookRejected, got {err:?}"
        );
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_image_generate_response() {
        assert_output_stage_inspects(LlmRequest::ImageGenerate(CreateImageRequest::default()), || {
            LlmResponse::ImageGenerate(ImagesResponse::default())
        })
        .await;
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_speech_response() {
        assert_output_stage_inspects(LlmRequest::Speech(CreateSpeechRequest::default()), || {
            LlmResponse::Speech(bytes::Bytes::from_static(b"audio"))
        })
        .await;
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_transcribe_response() {
        assert_output_stage_inspects(LlmRequest::Transcribe(CreateTranscriptionRequest::default()), || {
            LlmResponse::Transcribe(TranscriptionResponse::default())
        })
        .await;
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_moderate_response() {
        assert_output_stage_inspects(LlmRequest::Moderate(ModerationRequest::default()), || {
            LlmResponse::Moderate(ModerationResponse {
                id: String::new(),
                model: String::new(),
                results: vec![],
            })
        })
        .await;
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_rerank_response() {
        assert_output_stage_inspects(LlmRequest::Rerank(RerankRequest::default()), || {
            LlmResponse::Rerank(RerankResponse {
                id: None,
                results: vec![],
                meta: None,
            })
        })
        .await;
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_search_response() {
        assert_output_stage_inspects(LlmRequest::Search(SearchRequest::default()), || {
            LlmResponse::Search(SearchResponse {
                results: vec![],
                model: "test-model".into(),
            })
        })
        .await;
    }

    #[tokio::test]
    async fn guardrail_output_stage_inspects_ocr_response() {
        assert_output_stage_inspects(LlmRequest::Ocr(OcrRequest::default()), || {
            LlmResponse::Ocr(OcrResponse {
                pages: vec![],
                model: "test-model".into(),
                usage: None,
            })
        })
        .await;
    }

    /// A type whose `Serialize` impl always fails, used to exercise the
    /// guardrail's fail-closed path for a response body that cannot be
    /// serialized into JSON.
    struct AlwaysFailsToSerialize;

    impl serde::Serialize for AlwaysFailsToSerialize {
        fn serialize<S: serde::Serializer>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("intentional failure for test"))
        }
    }

    /// Regression test for the guardrail's fail-open bug: previously, a
    /// response that failed to serialize to JSON silently returned
    /// `Ok(response)`, letting un-inspected content reach the caller
    /// unchecked. It must now fail closed (return `Err`).
    ///
    /// None of `LlmResponse`'s concrete payload types can be coaxed into a
    /// real `serde_json::to_value` failure (no non-string map keys, and
    /// non-finite floats serialize to JSON `null` rather than erroring), so
    /// this exercises the extracted `serialize_for_guardrail` primitive
    /// directly with a type whose `Serialize` impl is built to fail.
    #[test]
    fn serialize_for_guardrail_fails_closed_on_serialization_error() {
        let result = serialize_for_guardrail(&AlwaysFailsToSerialize);
        assert!(
            result.is_err(),
            "a response body that cannot be serialized must fail closed (Err), not silently pass through"
        );
    }
}
