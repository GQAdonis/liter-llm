---
id: fixture_elixir_search_empty_results
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_search_async("{\"model\":\"brave/web-search\",\"query\":\"xyznonexistent12345xyz\"}")

```
