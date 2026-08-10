---
id: fixture_rust_multi_turn_conversation
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
    let request_json: serde_json::Value = serde_json::from_str(r#"{"messages":[{"content":"You are a helpful assistant.","role":"system"},{"content":"What is 2 + 2?","role":"user"},{"content":"2 + 2 equals 4.","role":"assistant"},{"content":"And what is 4 + 4?","role":"user"}],"model":"gpt-4"}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let _ = client.chat(request).await;
}

```
