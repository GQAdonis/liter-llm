---
id: fixture_ruby_edge_rerank_single_doc
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['Artificial intelligence is the simulation of human intelligence.'], model: 'rerank-v3.5', query: 'What is AI?'))

```
