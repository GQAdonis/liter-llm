---
id: fixture_ruby_finish_reason_content_filter
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Tell me something controversial', 'role' => 'user' }], model: 'gpt-4'))

```
