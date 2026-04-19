## NAPI-RS Implementation Details

### Native Performance

This binding uses NAPI-RS to provide native Node.js bindings with:

- **Zero-copy data transfer** between JavaScript and Rust layers
- **Async by default** — all LLM calls return Promises backed by Tokio
- **Binary-compatible** pre-built native modules across platforms
- **TypeScript definitions** generated automatically from Rust types

### Threading Model

- LLM calls are non-blocking — Tokio async runtime handles concurrency
- Streaming responses use Node.js async iterators backed by Tokio streams
- CPU-bound work runs in `spawn_blocking` to avoid blocking the event loop

### Memory Management

- API keys are wrapped in `secrecy::SecretString` and never logged
- Streaming buffers are released as soon as each chunk is consumed
- Provider registry is compiled into the binary — no runtime disk access
