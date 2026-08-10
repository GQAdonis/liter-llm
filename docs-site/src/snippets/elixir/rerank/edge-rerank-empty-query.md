---
id: fixture_elixir_edge_rerank_empty_query
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"Some document\",\"Another document\"],\"model\":\"rerank-v3.5\",\"query\":\"\"}")

```
