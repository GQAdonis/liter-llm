---
id: fixture_ruby_developer_message
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'You are a coding assistant. Always respond with concise code examples.', 'role' => 'developer' }, { 'content' => 'How do I reverse a string in Python?', 'role' => 'user' }], model: 'gpt-4'))

```
