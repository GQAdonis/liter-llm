---
id: fixture_ruby_error_moderate_auth_401
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.moderate(LiterLlm::ModerationRequest.new(input: 'Hello', model: 'omni-moderation-latest'))

```
