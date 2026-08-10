---
id: fixture_elixir_edge_file_large_upload
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_create_file_async("{\"file\":\"eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==\",\"filename\":\"large_training_data.jsonl\",\"purpose\":\"fine-tune\"}")

```
