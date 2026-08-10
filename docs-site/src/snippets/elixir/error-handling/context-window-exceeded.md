---
id: fixture_elixir_context_window_exceeded
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Very long prompt that exceeds the context window...\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
