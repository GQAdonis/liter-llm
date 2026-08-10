---
id: fixture_elixir_smoke_moderate_flagged
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_moderate_async("{\"input\":\"I want to hurt someone very badly\",\"model\":\"omni-moderation-latest\"}")

```
