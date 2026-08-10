---
id: fixture_elixir_bedrock_stream
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
chunks = LiterLlm.chat_stream("{\"max_tokens\":32,\"messages\":[{\"content\":\"Count to three, one word per response.\",\"role\":\"user\"}],\"model\":\"bedrock/anthropic.claude-3-sonnet-20240229-v1:0\",\"stream\":true}") |> Enum.to_list()

```
