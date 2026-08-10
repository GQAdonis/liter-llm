---
id: fixture_elixir_cache_stream_bypass
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"gpt-4\"}") |> Enum.to_list()

```
