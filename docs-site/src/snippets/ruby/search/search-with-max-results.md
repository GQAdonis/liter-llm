---
id: fixture_ruby_search_with_max_results
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.search(LiterLlm::SearchRequest.new(max_results: 2, model: 'brave/web-search', query: 'Rust programming'))

```
