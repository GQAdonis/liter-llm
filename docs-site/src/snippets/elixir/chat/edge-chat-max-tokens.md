---
id: fixture_elixir_edge_chat_max_tokens
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":1,\"messages\":[{\"content\":\"Write a story\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
