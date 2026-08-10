---
id: fixture_ruby_error_rerank_bad_request
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['doc1'], model: 'nonexistent-rerank', query: 'test'))

```
