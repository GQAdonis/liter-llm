---
id: fixture_ruby_smoke_create_response
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_response(LiterLlm::CreateResponseRequest.new(input: 'Explain quantum computing in one sentence.', model: 'gpt-4o'))

```
