---
id: fixture_ruby_content_policy_violation
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Generate harmful content', 'role' => 'user' }], model: 'gpt-4'))

```
