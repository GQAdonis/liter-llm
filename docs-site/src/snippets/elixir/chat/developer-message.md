---
id: fixture_elixir_developer_message
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"You are a coding assistant. Always respond with concise code examples.\",\"role\":\"developer\"},{\"content\":\"How do I reverse a string in Python?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
