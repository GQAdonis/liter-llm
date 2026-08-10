---
id: fixture_elixir_proxy_chat_basic
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Say hello\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o\"}")

```
