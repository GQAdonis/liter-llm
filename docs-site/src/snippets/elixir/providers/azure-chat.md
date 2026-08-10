---
id: fixture_elixir_azure_chat
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":16,\"messages\":[{\"content\":\"Say hello\",\"role\":\"user\"}],\"model\":\"azure/gpt-4\",\"temperature\":0}")

```
