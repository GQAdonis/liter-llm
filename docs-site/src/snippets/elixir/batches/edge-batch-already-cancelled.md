---
id: fixture_elixir_edge_batch_already_cancelled
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_cancel_batch_async("batch-cancelled001")

```
