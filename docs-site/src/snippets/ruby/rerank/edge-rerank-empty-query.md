---
id: fixture_ruby_edge_rerank_empty_query
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['Some document', 'Another document'], model: 'rerank-v3.5', query: ''))

```
