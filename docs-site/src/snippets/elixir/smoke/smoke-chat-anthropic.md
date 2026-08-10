---
id: fixture_elixir_smoke_chat_anthropic
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in exactly one word.\",\"role\":\"user\"}],\"model\":\"anthropic/claude-sonnet-4-20250514\"}")

```
