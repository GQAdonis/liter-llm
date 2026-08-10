---
id: fixture_elixir_smoke_moderate_batch
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_moderate_async("{\"input\":[\"Hello world\",\"Nice weather today\"],\"model\":\"omni-moderation-latest\"}")

```
