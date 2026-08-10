---
id: fixture_elixir_edge_chat_system_only
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"You are a helpful and concise assistant\",\"role\":\"system\"},{\"content\":\"Hi\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
