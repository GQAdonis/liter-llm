---
id: fixture_elixir_bedrock_chat
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_chat_async("{\"max_tokens\":16,\"messages\":[{\"content\":\"Say hello in one word.\",\"role\":\"user\"}],\"model\":\"bedrock/anthropic.claude-3-sonnet-20240229-v1:0\",\"temperature\":0}")

```
