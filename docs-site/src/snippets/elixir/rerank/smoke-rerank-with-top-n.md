---
id: fixture_elixir_smoke_rerank_with_top_n
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"Python is a programming language.\",\"Cats are cute animals.\",\"Python was created by Guido van Rossum.\",\"The sun is a star.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Python?\",\"top_n\":2}")

```
