---
id: fixture_ruby_edge_chat_system_only
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'You are a helpful and concise assistant', 'role' => 'system' }, { 'content' => 'Hi', 'role' => 'user' }], model: 'gpt-4'))

```
