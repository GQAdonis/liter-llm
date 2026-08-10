---
id: fixture_ruby_smoke_rerank_return_docs
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['Rust is a systems programming language.', 'Iron rusts when exposed to water.'], model: 'rerank-v3.5', query: 'What is Rust?', return_documents: true))

```
