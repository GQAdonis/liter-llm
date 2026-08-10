---
id: fixture_ruby_custom_base_url
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Hello', 'role' => 'user' }], model: 'local-model'))

```
