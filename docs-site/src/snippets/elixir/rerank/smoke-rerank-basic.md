---
id: fixture_elixir_smoke_rerank_basic
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"Machine learning is a subset of AI.\",\"The weather is sunny today.\",\"Deep learning uses neural networks.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is machine learning?\"}")

```
