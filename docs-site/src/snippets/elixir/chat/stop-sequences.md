---
id: fixture_elixir_stop_sequences
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"List items until you see STOP\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stop\":[\"STOP\",\"END\"]}")

```
