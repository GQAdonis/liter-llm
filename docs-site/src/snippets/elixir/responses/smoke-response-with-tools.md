---
id: fixture_elixir_smoke_response_with_tools
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_create_response_async("{\"input\":\"What is the weather in San Francisco?\",\"model\":\"gpt-4o\",\"tools\":[{\"description\":\"Get current weather for a location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"},\"type\":\"function\"}]}")

```
