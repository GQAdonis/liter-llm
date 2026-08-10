---
id: fixture_elixir_content_policy_violation
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Generate harmful content\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
