---
id: fixture_rust_smoke_chat_gemini
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
    let request_json: serde_json::Value = serde_json::from_str(r#"{"max_tokens":10,"messages":[{"content":"Say hello in exactly one word.","role":"user"}],"model":"gemini/gemini-2.5-flash-lite"}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let _ = client.chat(request).await;
}

```
