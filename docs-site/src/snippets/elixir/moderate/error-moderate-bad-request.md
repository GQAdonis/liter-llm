---
id: fixture_elixir_error_moderate_bad_request
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_moderate_async("{\"input\":\"Hello\",\"model\":\"nonexistent-moderation\"}")

```
