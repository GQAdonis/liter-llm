---
id: fixture_ruby_edge_response_empty_output
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_response(LiterLlm::CreateResponseRequest.new(input: '', model: 'gpt-4o'))

```
