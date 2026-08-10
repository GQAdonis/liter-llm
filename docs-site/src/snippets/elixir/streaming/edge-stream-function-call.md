---
id: fixture_elixir_edge_stream_function_call
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"What's the weather?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tools\":[{\"function\":{\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"city\":{\"type\":\"string\"}},\"type\":\"object\"}},\"type\":\"function\"}]}") |> Enum.to_list()

```
