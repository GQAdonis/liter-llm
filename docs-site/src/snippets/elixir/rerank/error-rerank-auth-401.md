---
id: fixture_elixir_error_rerank_auth_401
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"doc1\"],\"model\":\"rerank-v3.5\",\"query\":\"test\"}")

```
