---
id: fixture_ruby_error_response_bad_request
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_response(LiterLlm::CreateResponseRequest.new(input: 'Hello', model: 'nonexistent-model'))

```
