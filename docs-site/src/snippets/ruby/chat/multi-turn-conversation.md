---
id: fixture_ruby_multi_turn_conversation
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'You are a helpful assistant.', 'role' => 'system' }, { 'content' => 'What is 2 + 2?', 'role' => 'user' }, { 'content' => '2 + 2 equals 4.', 'role' => 'assistant' }, { 'content' => 'And what is 4 + 4?', 'role' => 'user' }], model: 'gpt-4'))

```
