use axum::Json;
use serde::Serialize;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};

/// OpenAI-compatible error response for OpenAPI documentation.
#[derive(Serialize, ToSchema)]
pub struct ProxyErrorBody {
    pub error: ProxyErrorDetail,
}

/// Detail within an OpenAI-compatible error response.
#[derive(Serialize, ToSchema)]
pub struct ProxyErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme("bearer_auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)));
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "liter-llm Proxy",
        version = "1.0.0",
        description = "OpenAI-compatible LLM proxy server — model routing, virtual keys, rate limiting, cost tracking.",
        license(name = "MIT"),
    ),
    servers(
        (url = "/", description = "Default server"),
    ),
    paths(
        crate::routes::chat::chat_completions,
        crate::routes::embeddings::create_embedding,
        crate::routes::models::list_models,
        crate::routes::images::create_image,
        crate::routes::audio::create_speech,
        crate::routes::audio::create_transcription,
        crate::routes::moderations::create_moderation,
        crate::routes::rerank::rerank,
        crate::routes::search::search,
        crate::routes::ocr::ocr,
        crate::routes::files::create_file,
        crate::routes::files::list_files,
        crate::routes::files::retrieve_file,
        crate::routes::files::delete_file,
        crate::routes::files::file_content,
        crate::routes::batches::create_batch,
        crate::routes::batches::list_batches,
        crate::routes::batches::retrieve_batch,
        crate::routes::batches::cancel_batch,
        crate::routes::responses::create_response,
        crate::routes::responses::retrieve_response,
        crate::routes::responses::cancel_response,
        crate::routes::health::health,
        crate::routes::health::liveness,
        crate::routes::health::readiness,
        crate::routes::health::healthz,
        crate::routes::health::readyz,
    ),
    components(schemas(
        ProxyErrorBody,
        ProxyErrorDetail,
        crate::routes::health::HealthResponse,
        crate::routes::health::LivenessResponse,
        crate::routes::health::ReadinessOkResponse,
        crate::routes::health::ReadinessFailResponse,
    )),
    tags(
        (name = "chat", description = "Chat completions"),
        (name = "embeddings", description = "Text embeddings"),
        (name = "models", description = "Model listing"),
        (name = "images", description = "Image generation"),
        (name = "audio", description = "Audio speech and transcription"),
        (name = "moderations", description = "Content moderation"),
        (name = "rerank", description = "Document reranking"),
        (name = "search", description = "Web and document search"),
        (name = "ocr", description = "Optical character recognition"),
        (name = "files", description = "File management"),
        (name = "batches", description = "Batch processing"),
        (name = "responses", description = "Response management"),
        (name = "health", description = "Health checks"),
    ),
    security(("bearer_auth" = [])),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

/// GET /openapi.json
///
/// Returns the OpenAPI specification for the proxy API.
pub async fn openapi_schema() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

    use serde_json::Value;

    use super::*;

    /// Recursively collect every `$ref` string found anywhere in `value`.
    ///
    /// ~keep Deliberately generic instead of an enumerated list of JSON-Schema
    /// containers (`properties`, `items`, `additionalProperties`, `allOf`,
    /// `oneOf`, `anyOf`, `prefixItems`, ...): every one of those keywords is
    /// just an ordinary object member or array element in the *serialized*
    /// document, so recursing into every object value and every array
    /// element unconditionally is a strict superset of walking them by name.
    /// It also finds refs inside combinations an enumerated list would have
    /// to special-case, e.g. an `anyOf` nested inside an array's `items`. A
    /// keyword-enumeration walker silently stops finding refs the moment one
    /// keyword is missed (exactly how the sibling project's 23 refs went
    /// undetected); this one structurally cannot miss a container.
    fn collect_refs(value: &Value, refs: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                if let Some(Value::String(r)) = map.get("$ref") {
                    refs.push(r.clone());
                }
                for v in map.values() {
                    collect_refs(v, refs);
                }
            }
            Value::Array(items) => {
                for v in items {
                    collect_refs(v, refs);
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }

    /// Every `$ref` emitted anywhere in the served OpenAPI document must resolve
    /// to a schema actually registered in `components.schemas`.
    ///
    /// ~keep This builds `ApiDoc::openapi()` and serializes it exactly the way
    /// `openapi_schema()` does for the live `/openapi.json` handler (same
    /// `ApiDoc::openapi()` call, same `Serialize` impl via `serde_json`), so
    /// there is no divergence between what this test inspects and what a
    /// client fetching `/openapi.json` receives — no HTTP/router harness
    /// needed to exercise the real artifact.
    #[test]
    fn every_ref_in_the_openapi_document_resolves_to_a_registered_schema() {
        let document = ApiDoc::openapi();
        let json = serde_json::to_value(&document).expect("ApiDoc::openapi() must serialize to JSON");

        let schemas = json["components"]["schemas"]
            .as_object()
            .expect("document must have a components.schemas object");

        let paths = json["paths"].as_object().expect("document must have a paths object");
        assert!(
            !paths.is_empty(),
            "expected at least one path in the OpenAPI document, found none"
        );

        let mut refs = Vec::new();
        collect_refs(&json, &mut refs);

        // ~keep Guard against a vacuous pass: if the document silently ended up
        // ~keep empty (build failure swallowed, serialization shape changed, a
        // ~keep refactor moved `components(schemas(...))` out from under this file),
        // ~keep the loop below would find zero refs and trivially "pass" having
        // ~keep checked nothing. Assert non-trivial lower bounds instead.
        // ~keep
        // ~keep 6 is the exact count of schemas currently wired into
        // ~keep `components(schemas(...))` above (ProxyErrorBody, ProxyErrorDetail,
        // ~keep HealthResponse, LivenessResponse, ReadinessOkResponse,
        // ~keep ReadinessFailResponse) — a drop below that means a schema was
        // ~keep silently dropped from registration.
        assert!(
            schemas.len() >= 6,
            "expected at least 6 registered component schemas, found {}: {:?}",
            schemas.len(),
            schemas.keys().collect::<Vec<_>>()
        );

        // ~keep 20 is well below the ~140 `$ref`s this document currently emits
        // ~keep (every error response across ~24 path operations references
        // ~keep ProxyErrorBody, plus the health schemas and ProxyErrorBody's own
        // ~keep nested reference to ProxyErrorDetail), so a legitimate future trim
        // ~keep of error-response schemas won't make this brittle — but it is far
        // ~keep enough above zero that a walk which silently found nothing cannot
        // ~keep pass.
        assert!(
            refs.len() >= 20,
            "expected at least 20 `$ref`s across the serialized OpenAPI document, found {}. \
             A near-zero count usually means the walk found nothing to check, not that the \
             document is genuinely free of refs.",
            refs.len()
        );

        let schema_names: HashSet<&str> = schemas.keys().map(String::as_str).collect();

        let mut unresolved: BTreeSet<String> = BTreeSet::new();
        for r in &refs {
            match r.strip_prefix("#/components/schemas/") {
                Some(name) if schema_names.contains(name) => {}
                Some(_) => {
                    unresolved.insert(r.clone());
                }
                None => {
                    // ~keep `components()` on `ApiDoc` only ever registers
                    // ~keep `schemas(...)` (no `responses`/`parameters`/`headers`),
                    // ~keep so there is nothing else in this document for a $ref to
                    // ~keep legitimately point at. Surface any other shape rather
                    // ~keep than silently skipping it.
                    unresolved.insert(format!("{r} (not a #/components/schemas/... ref)"));
                }
            }
        }

        assert!(
            unresolved.is_empty(),
            "found {} unresolved $ref(s) in the OpenAPI document, naming each: {:#?}",
            unresolved.len(),
            unresolved
        );
    }
}
