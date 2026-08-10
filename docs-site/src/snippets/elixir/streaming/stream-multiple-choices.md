---
id: fixture_elixir_stream_multiple_choices
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"gpt-4o\",\"n\":2,\"stream\":true}") |> Enum.to_list()

```
