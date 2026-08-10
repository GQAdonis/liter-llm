---
id: fixture_elixir_error_image_bad_request
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A cat\",\"size\":\"9999x9999\"}")

```
