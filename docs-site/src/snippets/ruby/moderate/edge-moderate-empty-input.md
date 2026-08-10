---
id: fixture_ruby_edge_moderate_empty_input
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.moderate(LiterLlm::ModerationRequest.new(input: '', model: 'omni-moderation-latest'))

```
