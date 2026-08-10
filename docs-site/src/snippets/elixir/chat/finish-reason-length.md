---
id: fixture_elixir_finish_reason_length
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":5,\"messages\":[{\"content\":\"Tell me a long story\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
