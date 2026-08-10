---
id: fixture_ruby_smoke_moderate_batch
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.moderate(LiterLlm::ModerationRequest.new(input: ['Hello world', 'Nice weather today'], model: 'omni-moderation-latest'))

```
