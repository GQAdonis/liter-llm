---
id: fixture_elixir_anthropic_error_auth
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"anthropic/claude-3-5-sonnet-20241022\"}")

```
