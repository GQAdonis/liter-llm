---
id: fixture_elixir_edge_moderate_empty_input
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_moderate_async("{\"input\":\"\",\"model\":\"omni-moderation-latest\"}")

```
