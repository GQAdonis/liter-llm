---
id: fixture_ruby_error_rerank_auth_401
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['doc1'], model: 'rerank-v3.5', query: 'test'))

```
