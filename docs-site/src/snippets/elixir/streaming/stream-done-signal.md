---
id: fixture_elixir_stream_done_signal
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Say done\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stream\":true}") |> Enum.to_list()

```
