---
id: fixture_elixir_error_rerank_bad_request
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"doc1\"],\"model\":\"nonexistent-rerank\",\"query\":\"test\"}")

```
