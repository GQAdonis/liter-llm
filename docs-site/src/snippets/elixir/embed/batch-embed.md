---
id: fixture_elixir_batch_embed
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"input\":[\"Hello\",\"World\"],\"model\":\"text-embedding-3-small\"}")

```
