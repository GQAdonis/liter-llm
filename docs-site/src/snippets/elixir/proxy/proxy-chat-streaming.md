---
id: fixture_elixir_proxy_chat_streaming
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Count to 3\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o\",\"stream\":true}") |> Enum.to_list()

```
