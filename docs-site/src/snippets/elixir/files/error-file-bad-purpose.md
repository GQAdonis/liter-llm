---
id: fixture_elixir_error_file_bad_purpose
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_create_file_async("{\"file\":\"data.jsonl\",\"purpose\":\"invalid-purpose\"}")

```
