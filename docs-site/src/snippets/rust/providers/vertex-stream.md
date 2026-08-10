---
id: fixture_rust_vertex_stream
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use liter_llm::BatchClient;
use liter_llm::FileClient;
use liter_llm::LlmClient;
use liter_llm::ResponseClient;

#[tokio::main]
async fn main() {
    let request_json: serde_json::Value = serde_json::from_str(r#"{"max_tokens":32,"messages":[{"content":"Count to three, one word per response.","role":"user"}],"model":"vertex_ai/gemini-2.0-flash","stream":true}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let stream = client.chat_stream(request).await.expect("call failed");
    let chunks: Vec<_> = tokio_stream::StreamExt::collect::<Vec<_>>(stream).await
        .into_iter()
        .map(|r| r.expect("stream item failed"))
        .collect();
}

```
