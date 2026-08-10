---
id: fixture_rust_error_response_not_found
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
    let response_id = r#"resp-nonexistent"#;
    let client = liter_llm::create_client("test-key".to_string(), None, None, None, None).unwrap();
    let _ = client.retrieve_response(response_id).await;
}

```
