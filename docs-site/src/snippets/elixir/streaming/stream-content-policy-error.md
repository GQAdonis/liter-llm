---
id: fixture_elixir_stream_content_policy_error
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Generate harmful content\",\"role\":\"user\"}],\"model\":\"gpt-4o\",\"stream\":true}") |> Enum.to_list()

```
