---
id: fixture_elixir_binding_api_parity
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Contract test\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o\"}")

```
