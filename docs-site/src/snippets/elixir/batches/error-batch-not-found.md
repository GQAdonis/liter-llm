---
id: fixture_elixir_error_batch_not_found
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_retrieve_batch_async("batch-nonexistent")

```
