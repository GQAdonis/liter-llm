---
id: fixture_elixir_edge_rerank_single_doc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"Artificial intelligence is the simulation of human intelligence.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is AI?\"}")

```
