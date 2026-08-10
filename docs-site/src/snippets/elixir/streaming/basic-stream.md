---
id: fixture_elixir_basic_stream
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Count to 3\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stream\":true}") |> Enum.to_list()

```
