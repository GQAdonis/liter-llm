---
id: fixture_elixir_embed_with_dimensions
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"dimensions\":256,\"input\":\"Hello world\",\"model\":\"text-embedding-3-small\"}")

```
