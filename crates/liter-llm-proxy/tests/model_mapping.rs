#[path = "common/test_proxy.rs"]
mod test_proxy;

#[ctor::ctor(unsafe)]
fn initialize_crypto() {
    liter_llm::ensure_crypto_provider();
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use liter_llm_proxy::config::ProxyConfig;
use serde_json::{Value, json};
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn config(name: &str, provider_model: &str, upstream: &str) -> String {
    format!(
        r#"[general]
master_key = "synthetic-master"
max_retries = 0
[[models]]
name = "{name}"
provider_model = "{provider_model}"
api_key = "synthetic-upstream"
base_url = "{upstream}"
"#
    )
}

fn request(route: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(route)
        .header("authorization", "Bearer synthetic-master")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("valid local proxy request")
}

fn chat_response(model: &str, content: &str) -> Value {
    json!({
        "id": "synthetic-completion", "object": "chat.completion", "created": 0, "model": model,
        "choices": [{"index": 0, "message": {"role": "assistant", "content": content}, "finish_reason": "stop"}],
        "usage": {"prompt_tokens": 1, "completion_tokens": 1, "total_tokens": 2}
    })
}

async fn chat_mapping(name: &str, stream: bool) {
    let upstream = MockServer::start().await;
    let body = if stream {
        let chunk = json!({"id":"synthetic-stream","object":"chat.completion.chunk","created":0,
            "model":"gpt-4o","choices":[{"index":0,"delta":{"content":"preserved"},"finish_reason":"stop"}]});
        ResponseTemplate::new(200)
            .insert_header("content-type", "text/event-stream")
            .set_body_string(format!("data: {chunk}\n\ndata: [DONE]\n\n"))
    } else {
        ResponseTemplate::new(200).set_body_json(chat_response("gpt-4o", "preserved"))
    };
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(body)
        .expect(1)
        .mount(&upstream)
        .await;
    let config = ProxyConfig::from_toml_str(&config(name, "openai/gpt-4o", &upstream.uri())).expect("proxy config");
    let proxy = test_proxy::TestProxy::with_config(config);
    let response = proxy
        .router()
        .oneshot(request(
            "/v1/chat/completions",
            json!({"model":name,"messages":[{"role":"user","content":"mapping input"}],"stream":stream}),
        ))
        .await
        .expect("proxy dispatch");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.expect("response bytes").to_bytes();
    let text = std::str::from_utf8(&bytes).expect("UTF-8 completion");
    assert!(text.contains("preserved"));
    assert!(text.contains("gpt-4o"));
    let received = upstream.received_requests().await.expect("recorded requests");
    assert_eq!(received.len(), 1);
    let transmitted: Value = received[0].body_json().expect("JSON upstream body");
    assert_eq!(transmitted["model"], "gpt-4o");
    assert_eq!(transmitted["messages"][0]["content"], "mapping input");
    assert_eq!(transmitted["stream"], stream);
}

#[tokio::test]
async fn unary_chat_sends_provider_model_for_named_alias() {
    chat_mapping("friendly-model", false).await;
}

#[tokio::test]
async fn streaming_chat_sends_provider_model_for_named_alias() {
    chat_mapping("friendly-model", true).await;
}

#[tokio::test]
async fn unary_chat_preserves_fully_qualified_client_name() {
    chat_mapping("openai/gpt-4o", false).await;
}

#[tokio::test]
async fn streaming_chat_preserves_fully_qualified_client_name() {
    chat_mapping("openai/gpt-4o", true).await;
}

#[tokio::test]
async fn deployment_pool_sends_each_selected_providers_model() {
    let openai = MockServer::start().await;
    let anthropic = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(chat_response("gpt-4o", "from-openai")))
        .expect(1)
        .mount(&openai)
        .await;
    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id":"synthetic-message","type":"message","role":"assistant","model":"claude-sonnet-4-20250514",
            "content":[{"type":"text","text":"from-anthropic"}],"stop_reason":"end_turn","stop_sequence":null,
            "usage":{"input_tokens":1,"output_tokens":1}
        })))
        .expect(1)
        .mount(&anthropic)
        .await;
    let toml = format!(
        "{}\n[[models]]\nname='pooled'\nprovider_model='anthropic/claude-sonnet-4-20250514'\napi_key='synthetic-anthropic'\nbase_url='{}'\n",
        config("pooled", "openai/gpt-4o", &openai.uri()),
        anthropic.uri()
    );
    let proxy = test_proxy::TestProxy::with_config(ProxyConfig::from_toml_str(&toml).expect("pooled config"));
    let mut outputs = Vec::new();
    for _ in 0..2 {
        let response = proxy
            .router()
            .oneshot(request(
                "/v1/chat/completions",
                json!({"model":"pooled","messages":[{"role":"user","content":"pooled input"}],"max_tokens":32}),
            ))
            .await
            .expect("pooled dispatch");
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("pooled response")
            .to_bytes();
        outputs.push(serde_json::from_slice::<Value>(&bytes).expect("JSON completion"));
    }
    for (server, model) in [(&openai, "gpt-4o"), (&anthropic, "claude-sonnet-4-20250514")] {
        let received = server.received_requests().await.expect("deployment requests");
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].body_json::<Value>().expect("deployment JSON")["model"],
            model
        );
    }
    outputs.sort_by_key(|output| output["model"].as_str().expect("response model").to_owned());
    assert_eq!(outputs[0]["model"], "claude-sonnet-4-20250514");
    assert_eq!(outputs[1]["model"], "gpt-4o");
}

#[tokio::test]
async fn embedding_alias_sends_configured_provider_model() {
    let upstream = MockServer::start().await;
    Mock::given(method("POST")).and(path("/embeddings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object":"list","model":"text-embedding-3-small","data":[{"object":"embedding","index":0,"embedding":[0.5,0.25]}],
            "usage":{"prompt_tokens":1,"total_tokens":1}
        }))).expect(1).mount(&upstream).await;
    let config = ProxyConfig::from_toml_str(&config("embed-alias", "openai/text-embedding-3-small", &upstream.uri()))
        .expect("embedding config");
    let proxy = test_proxy::TestProxy::with_config(config);
    let response = proxy
        .router()
        .oneshot(request(
            "/v1/embeddings",
            json!({"model":"embed-alias","input":"embedding input"}),
        ))
        .await
        .expect("embedding dispatch");
    assert_eq!(response.status(), StatusCode::OK);
    let received = upstream.received_requests().await.expect("embedding requests");
    assert_eq!(received.len(), 1);
    assert_eq!(
        received[0].body_json::<Value>().expect("embedding JSON")["model"],
        "text-embedding-3-small"
    );
}

async fn optional_model_mapping(
    route: &str,
    upstream_path: &str,
    alias: &str,
    model: &str,
    input: Value,
    output: Value,
) {
    let upstream = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(upstream_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(output.clone()))
        .expect(1)
        .mount(&upstream)
        .await;
    let config = ProxyConfig::from_toml_str(&config(alias, model, &upstream.uri())).expect("optional model config");
    let proxy = test_proxy::TestProxy::with_config(config);
    let response = proxy
        .router()
        .oneshot(request(route, input))
        .await
        .expect("optional model dispatch");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("optional model response")
        .to_bytes();
    assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&bytes));
    let observed: Value = serde_json::from_slice(&bytes).expect("JSON response");
    for key in ["id", "created", "model", "data"] {
        if let Some(expected) = output.get(key) {
            assert_eq!(&observed[key], expected);
        }
    }
    if let Some(results) = output.get("results") {
        assert_eq!(
            observed["results"].as_array().expect("moderation results").len(),
            results.as_array().expect("expected results").len()
        );
        assert_eq!(observed["results"][0]["flagged"], false);
    }
    let received = upstream.received_requests().await.expect("optional model request");
    assert_eq!(received.len(), 1);
    assert_eq!(
        received[0].body_json::<Value>().expect("optional request JSON")["model"],
        model.trim_start_matches("openai/")
    );
}

#[tokio::test]
async fn image_default_alias_sends_configured_model_when_request_omits_model() {
    optional_model_mapping(
        "/v1/images/generations",
        "/images/generations",
        "dall-e-3",
        "openai/gpt-image-1",
        json!({"prompt":"synthetic image","response_format":"b64_json"}),
        json!({"created":0,"data":[{"b64_json":"eA=="}]}),
    )
    .await;
}

#[tokio::test]
async fn moderation_default_alias_sends_configured_model_when_request_omits_model() {
    optional_model_mapping(
        "/v1/moderations",
        "/moderations",
        "text-moderation-stable",
        "openai/omni-moderation-latest",
        json!({"input":"synthetic moderation input"}),
        json!({"id":"synthetic-moderation","model":"omni-moderation-latest",
            "results":[{"flagged":false,"categories":{},"category_scores":{}}]}),
    )
    .await;
}
