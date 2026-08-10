---
id: fixture_ruby_smoke_rerank_with_top_n
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.rerank(LiterLlm::RerankRequest.new(documents: ['Python is a programming language.', 'Cats are cute animals.', 'Python was created by Guido van Rossum.', 'The sun is a star.'], model: 'rerank-v3.5', query: 'What is Python?', top_n: 2))

```
