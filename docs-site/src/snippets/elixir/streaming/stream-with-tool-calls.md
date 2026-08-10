---
id: fixture_elixir_stream_with_tool_calls
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"What is the weather in NYC?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stream\":true,\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city and state, e.g. New York, NY\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"}]}") |> Enum.to_list()

```
