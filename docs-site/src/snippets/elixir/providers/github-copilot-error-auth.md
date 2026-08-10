---
id: fixture_elixir_github_copilot_error_auth
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"github_copilot/gpt-4o\"}")

```
