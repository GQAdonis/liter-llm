---
id: fixture_elixir_error_response_bad_request
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_create_response_async("{\"input\":\"Hello\",\"model\":\"nonexistent-model\"}")

```
