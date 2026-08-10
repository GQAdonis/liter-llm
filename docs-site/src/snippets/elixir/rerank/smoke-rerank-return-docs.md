---
id: fixture_elixir_smoke_rerank_return_docs
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"Rust is a systems programming language.\",\"Iron rusts when exposed to water.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Rust?\",\"return_documents\":true}")

```
