---
id: fixture_elixir_error_image_auth_401
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A cat\",\"size\":\"1024x1024\"}")

```
