---
summary: Universal LM API client architecture, crate layout, and provider registry.
---

# Project Overview

Liter-LM is a universal LLM API client written in Rust with native bindings for 11 languages.

## Crate Layout

- `crates/liter-llm` — Core library: client, providers, types, HTTP layer, error handling
- `crates/liter-llm-py` — Python bindings (PyO3)
- `crates/liter-llm-node` — Node.js bindings (NAPI-RS)
- `crates/liter-llm-ffi` — C FFI layer for Go, Java, C#
- `crates/liter-llm-php` — PHP bindings (ext-php-rs)
- `crates/liter-llm-wasm` — WebAssembly bindings (wasm-bindgen)
- `crates/liter-llm-proxy` — OpenAI-compatible proxy server (axum, Tower middleware, OpenAPI 3.1)
- `crates/liter-llm-cli` — CLI binary (`liter-llm api`, `liter-llm mcp`)

## Language Packages

- `packages/go` — Go module wrapping the C FFI
- `packages/java` — Java wrapper via Panama FFM
- `packages/csharp` — .NET wrapper via P/Invoke
- `packages/ruby` — Ruby gem (Magnus)
- `packages/elixir` — Elixir package (Rustler NIF)

## Provider Registry

`schemas/providers.json` contains 142+ LLM provider configurations compiled into the binary. Each entry defines: base URL, auth header format, model prefixes, and parameter mappings.

## Proxy Server

`crates/liter-llm-proxy` provides an OpenAI-compatible HTTP proxy with 22 endpoints, virtual API keys, model routing, rate limiting, cost tracking, and SSE streaming. Built on axum with the same Tower middleware stack as the core library. Served via the `liter-llm` CLI binary.

## Docker

`docker/Dockerfile` builds a 35MB image using Chainguard runtime. Published to `ghcr.io/kreuzberg-dev/liter-llm`. Entrypoint: `liter-llm api --host 0.0.0.0 --port 4000`.

## E2E Test Generation

`tools/e2e-generator` generates language-specific E2E tests from fixtures in `/fixtures/`. Tests cover: configuration, error handling, streaming, tool calling, and type validation.

## Key Commands

- `task build` — Build all crates
- `task test` — Run test suites
- `task lint` — Lint via prek
- `task generate:types` — Regenerate types from JSON schemas
- `task e2e:generate:all` — Regenerate E2E tests for all languages
- `task proxy:test` — Run proxy unit + integration tests
- `task proxy:schemathesis` — Run schemathesis API contract tests against Docker image
- `liter-llm api --config <path>` — Start proxy server
- `liter-llm mcp --transport stdio` — Start MCP tool server
