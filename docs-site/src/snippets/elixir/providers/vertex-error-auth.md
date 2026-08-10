---
id: fixture_elixir_vertex_error_auth
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"vertex_ai/gemini-2.0-flash\"}")

```
