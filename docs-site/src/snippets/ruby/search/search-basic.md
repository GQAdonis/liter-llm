---
id: fixture_ruby_search_basic
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.search(LiterLlm::SearchRequest.new(model: 'brave/web-search', query: 'What is Rust programming language?'))

```
