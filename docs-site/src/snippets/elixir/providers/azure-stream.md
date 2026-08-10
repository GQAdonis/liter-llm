---
id: fixture_elixir_azure_stream
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Count to 3\",\"role\":\"user\"}],\"model\":\"azure/gpt-4\",\"stream\":true,\"temperature\":0}") |> Enum.to_list()

```
