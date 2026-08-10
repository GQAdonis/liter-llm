---
id: fixture_ruby_basic_chat
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Say hello', 'role' => 'user' }], model: 'gpt-4', temperature: 0))

```
