---
id: fixture_elixir_smoke_image_basic
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A white cat sitting on a windowsill\",\"size\":\"1024x1024\"}")

```
