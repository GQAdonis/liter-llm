---
id: fixture_elixir_tool_choice_specific
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":{\"function\":{\"name\":\"get_weather\"},\"type\":\"function\"},\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city name\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"},{\"function\":{\"description\":\"Search the web for information\",\"name\":\"search_web\",\"parameters\":{\"properties\":{\"query\":{\"description\":\"The search query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}},\"type\":\"function\"}]}")

```
