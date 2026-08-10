---
id: fixture_elixir_local_provider_vllm
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"messages\":[{\"content\":\"Hello\",\"role\":\"user\"}],\"model\":\"vllm/meta-llama/Llama-3.2-1B\"}")

```
