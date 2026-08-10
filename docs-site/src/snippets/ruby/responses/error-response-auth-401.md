---
id: fixture_ruby_error_response_auth_401
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_response(LiterLlm::CreateResponseRequest.new(input: 'Hello', model: 'gpt-4o'))

```
