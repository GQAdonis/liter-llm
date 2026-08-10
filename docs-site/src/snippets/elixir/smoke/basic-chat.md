---
id: fixture_elixir_basic_chat
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Say hello\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"temperature\":0}")

```
