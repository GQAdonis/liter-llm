---
id: fixture_elixir_local_embed_ollama
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_embed_async("{\"input\":\"The quick brown fox jumps over the lazy dog\",\"model\":\"ollama/all-minilm\"}")

```
