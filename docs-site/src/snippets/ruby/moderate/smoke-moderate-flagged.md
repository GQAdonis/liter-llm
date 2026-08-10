---
id: fixture_ruby_smoke_moderate_flagged
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.moderate(LiterLlm::ModerationRequest.new(input: 'I want to hurt someone very badly', model: 'omni-moderation-latest'))

```
