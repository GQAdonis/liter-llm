---
id: fixture_elixir_embed_encoding_format
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"encoding_format\":\"float\",\"input\":\"Test input\",\"model\":\"text-embedding-3-small\"}")

```
