---
id: fixture_elixir_smoke_create_file
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_create_file_async("{\"file\":\"eyJwcm9tcHQiOiAiaGVsbG8ifQo=\",\"filename\":\"training_data.jsonl\",\"purpose\":\"fine-tune\"}")

```
