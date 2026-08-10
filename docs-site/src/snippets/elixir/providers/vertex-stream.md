---
id: fixture_elixir_vertex_stream
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"max_tokens\":32,\"messages\":[{\"content\":\"Count to three, one word per response.\",\"role\":\"user\"}],\"model\":\"vertex_ai/gemini-2.0-flash\",\"stream\":true}") |> Enum.to_list()

```
