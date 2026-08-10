---
id: fixture_ruby_finish_reason_length
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 5, messages: [{ 'content' => 'Tell me a long story', 'role' => 'user' }], model: 'gpt-4'))

```
