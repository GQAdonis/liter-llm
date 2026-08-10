---
id: fixture_elixir_smoke_cache_memory
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":5,\"messages\":[{\"content\":\"What is 2+2? Answer with just the number.\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o-mini\"}")

```
