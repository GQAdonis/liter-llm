---
id: fixture_ruby_binding_api_parity
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Contract test', 'role' => 'user' }], model: 'openai/gpt-4o'))

```
