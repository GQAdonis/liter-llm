---
id: fixture_elixir_error_file_not_found
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_retrieve_file_async("file-nonexistent")

```
