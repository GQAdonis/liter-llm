---
id: fixture_elixir_smoke_streaming_openai
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"max_tokens\":50,\"messages\":[{\"content\":\"Count from 1 to 5.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}") |> Enum.to_list()

```
