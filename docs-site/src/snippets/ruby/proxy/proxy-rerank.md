---
id: fixture_ruby_proxy_rerank
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['Deep learning is a subset of machine learning using neural networks.', 'The stock market closed higher today.'], model: 'rerank-v3.5', query: 'What is deep learning?'))

```
