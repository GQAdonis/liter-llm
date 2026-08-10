---
id: fixture_ruby_edge_moderate_all_categories
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.moderate(LiterLlm::ModerationRequest.new(input: 'Extremely harmful content targeting multiple categories', model: 'omni-moderation-latest'))

```
