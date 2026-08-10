---
id: fixture_ruby_proxy_moderation
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.moderate(LiterLlm::ModerationRequest.new(input: 'The weather is nice today.', model: 'omni-moderation-latest'))

```
