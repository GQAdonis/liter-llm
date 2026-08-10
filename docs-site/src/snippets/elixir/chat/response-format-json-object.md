---
id: fixture_elixir_response_format_json_object
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Respond with JSON only.\",\"role\":\"system\"},{\"content\":\"Give me a user object with name and age fields.\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"type\":\"json_object\"}}")

```
