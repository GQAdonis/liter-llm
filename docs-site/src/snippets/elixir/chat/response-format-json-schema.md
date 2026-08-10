---
id: fixture_elixir_response_format_json_schema
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"What is the temperature in Paris today?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"json_schema\":{\"name\":\"weather\",\"schema\":{\"properties\":{\"temp\":{\"type\":\"number\"}},\"required\":[\"temp\"],\"type\":\"object\"}},\"type\":\"json_schema\"}}")

```
