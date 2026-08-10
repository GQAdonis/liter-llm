---
id: fixture_elixir_all_message_types
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"},{\"content\":null,\"role\":\"assistant\",\"tool_calls\":[{\"function\":{\"arguments\":\"{\\\"location\\\": \\\"Paris, France\\\"}\",\"name\":\"get_weather\"},\"id\":\"call_xyz789\",\"type\":\"function\"}]},{\"content\":\"{\\\"temperature\\\": 18, \\\"unit\\\": \\\"celsius\\\", \\\"description\\\": \\\"Partly cloudy\\\"}\",\"role\":\"tool\",\"tool_call_id\":\"call_xyz789\"}],\"model\":\"gpt-4\"}")

```
