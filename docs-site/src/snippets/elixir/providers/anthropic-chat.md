---
id: fixture_elixir_anthropic_chat
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":16,\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"Say hello in one word.\",\"role\":\"user\"}],\"model\":\"anthropic/claude-3-5-sonnet-20241022\",\"temperature\":0}")

```
