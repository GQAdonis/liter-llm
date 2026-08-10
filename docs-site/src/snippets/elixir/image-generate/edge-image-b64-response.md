---
id: fixture_elixir_edge_image_b64_response
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_image_generate_async("{\"model\":\"dall-e-3\",\"n\":1,\"prompt\":\"A blue circle\",\"response_format\":\"b64_json\",\"size\":\"1024x1024\"}")

```
