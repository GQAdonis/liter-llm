---
id: fixture_elixir_proxy_image_generate
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A sunset over the ocean\",\"size\":\"1024x1024\"}")

```
