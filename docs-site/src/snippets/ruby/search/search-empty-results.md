---
id: fixture_ruby_search_empty_results
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.search(LiterLlm::SearchRequest.new(model: 'brave/web-search', query: 'xyznonexistent12345xyz'))

```
