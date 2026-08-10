---
id: fixture_elixir_edge_embed_empty_input
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"input\":\"\",\"model\":\"text-embedding-3-small\"}")

```
