---
id: fixture_elixir_search_with_max_results
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_search_async("{\"max_results\":2,\"model\":\"brave/web-search\",\"query\":\"Rust programming\"}")

```
