---
id: fixture_elixir_smoke_image_with_size
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A sunset over mountains\",\"size\":\"1792x1024\"}")

```
