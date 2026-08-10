---
id: fixture_elixir_smoke_embed_openai
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"input\":[\"Hello world\"],\"model\":\"openai/text-embedding-3-small\"}")

```
