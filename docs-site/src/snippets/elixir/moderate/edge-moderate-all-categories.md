---
id: fixture_elixir_edge_moderate_all_categories
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_moderate_async("{\"input\":\"Extremely harmful content targeting multiple categories\",\"model\":\"omni-moderation-latest\"}")

```
