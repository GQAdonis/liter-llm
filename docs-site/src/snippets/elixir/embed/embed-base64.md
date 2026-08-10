---
id: fixture_elixir_embed_base64
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"encoding_format\":\"base64\",\"input\":\"Test input\",\"model\":\"text-embedding-3-small\"}")

```
