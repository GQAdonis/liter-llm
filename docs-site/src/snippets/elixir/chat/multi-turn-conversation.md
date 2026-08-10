---
id: fixture_elixir_multi_turn_conversation
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is 2 + 2?\",\"role\":\"user\"},{\"content\":\"2 + 2 equals 4.\",\"role\":\"assistant\"},{\"content\":\"And what is 4 + 4?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
