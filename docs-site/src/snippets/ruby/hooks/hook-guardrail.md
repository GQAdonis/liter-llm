---
id: fixture_ruby_hook_guardrail
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Hello', 'role' => 'user' }], model: 'gpt-4'))

```
