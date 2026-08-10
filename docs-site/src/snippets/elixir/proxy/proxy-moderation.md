---
id: fixture_elixir_proxy_moderation
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_moderate_async("{\"input\":\"The weather is nice today.\",\"model\":\"omni-moderation-latest\"}")

```
