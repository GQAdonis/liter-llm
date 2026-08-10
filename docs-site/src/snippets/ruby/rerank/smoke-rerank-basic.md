---
id: fixture_ruby_smoke_rerank_basic
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['Machine learning is a subset of AI.', 'The weather is sunny today.', 'Deep learning uses neural networks.'], model: 'rerank-v3.5', query: 'What is machine learning?'))

```
