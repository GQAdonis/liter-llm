---
id: fixture_elixir_finish_reason_content_filter
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Tell me something controversial\",\"role\":\"user\"}],\"model\":\"gpt-4\"}")

```
