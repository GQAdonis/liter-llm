---
id: fixture_elixir_smoke_provider_routing
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":5,\"messages\":[{\"content\":\"Say hi.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}")

```
