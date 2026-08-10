---
id: fixture_ruby_anthropic_chat
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 16, messages: [{ 'content' => 'You are a helpful assistant.', 'role' => 'system' }, { 'content' => 'Say hello in one word.', 'role' => 'user' }], model: 'anthropic/claude-3-5-sonnet-20241022', temperature: 0))

```
