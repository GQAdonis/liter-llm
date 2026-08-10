---
id: fixture_elixir_local_stream_ollama
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"messages\":[{\"content\":\"Count to 3\",\"role\":\"user\"}],\"model\":\"ollama/qwen2:0.5b\",\"stream\":true}") |> Enum.to_list()

```
