---
id: fixture_ruby_stop_sequences
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'List items until you see STOP', 'role' => 'user' }], model: 'gpt-4', stop: ['STOP', 'END']))

```
