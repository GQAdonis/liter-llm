---
id: fixture_elixir_stream_with_usage
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Say hi\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stream\":true,\"stream_options\":{\"include_usage\":true}}") |> Enum.to_list()

```
