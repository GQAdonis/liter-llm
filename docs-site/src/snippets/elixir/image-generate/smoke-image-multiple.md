---
id: fixture_elixir_smoke_image_multiple
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-2\",\"n\":3,\"prompt\":\"A red bicycle\",\"size\":\"256x256\"}")

```
