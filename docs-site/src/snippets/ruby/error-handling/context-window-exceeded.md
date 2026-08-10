---
id: fixture_ruby_context_window_exceeded
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Very long prompt that exceeds the context window...', 'role' => 'user' }], model: 'gpt-4'))

```
