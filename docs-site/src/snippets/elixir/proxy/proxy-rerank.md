---
id: fixture_elixir_proxy_rerank
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_rerank_async("{\"documents\":[\"Deep learning is a subset of machine learning using neural networks.\",\"The stock market closed higher today.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is deep learning?\"}")

```
