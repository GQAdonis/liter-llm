---
name: llm-client-engineer
description: LLM provider integration and API client development
model: haiku
---

When working on liter-llm:

- The provider trait and implementations live in `crates/liter-llm/src/provider/`. Each provider (Anthropic, Azure, Bedrock, Cohere, Google AI, Mistral, Vertex, custom) maps vendor-specific API formats to the unified type system. New providers implement the provider trait and register in `mod.rs`.
- Client configuration and managed client logic live in `crates/liter-llm/src/client/` with `config.rs` (runtime config), `config_file.rs` (TOML file loading), and `managed.rs` (connection lifecycle).
- The unified type system in `crates/liter-llm/src/types/` covers chat completions, embeddings, audio, images, models, moderation, reranking, OCR, batch, files, responses, and search. Types are the canonical API surface shared across all bindings.
- HTTP layer in `crates/liter-llm/src/http/` handles request construction (`request.rs`), SSE streaming (`streaming.rs`, `eventstream.rs`), and retry logic (`retry.rs`).
- Auth providers in `crates/liter-llm/src/auth/` support Azure AD OAuth2, Google Vertex JWT, and AWS Bedrock STS web identity. Each is feature-gated.
- Tower middleware stack in `crates/liter-llm/src/tower/` provides rate limiting, caching (in-memory and OpenDAL-backed), fallback routing, health checks, cost tracking, budget enforcement, cooldown, hooks, and tracing layers.
- Token counting via HuggingFace tokenizers in `tokenizer.rs` (feature-gated behind `tokenizer`).
- Cost calculation in `cost.rs`.
- Proxy server in `crates/liter-llm-proxy/` provides a standalone OpenAI-compatible API proxy with route handling, MCP integration, streaming, service pooling, and OpenAPI docs.
- Bindings: Python (`liter-llm-py`), Node.js (`liter-llm-node`), PHP (`liter-llm-php`), WASM (`liter-llm-wasm`), C FFI (`liter-llm-ffi`), with shared binding core in `liter-llm-bindings-core`.
- CLI entry point in `liter-llm-cli`.
- All provider logic, type mapping, and middleware stays in Rust core. Bindings are thin wrappers only.
