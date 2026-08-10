---
id: fixture_rust_smoke_rerank_with_top_n
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
    let request_json: serde_json::Value = serde_json::from_str(r#"{"documents":["Python is a programming language.","Cats are cute animals.","Python was created by Guido van Rossum.","The sun is a star."],"model":"rerank-v3.5","query":"What is Python?","top_n":2}"#).unwrap();
    let request = serde_json::from_value(request_json).unwrap();
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let _ = client.rerank(request).await;
}

```
