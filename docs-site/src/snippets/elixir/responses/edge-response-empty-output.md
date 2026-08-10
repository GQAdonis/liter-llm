---
id: fixture_elixir_edge_response_empty_output
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_create_response_async("{\"input\":\"\",\"model\":\"gpt-4o\"}")

```
