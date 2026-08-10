---
id: fixture_ruby_proxy_auth_missing
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Hello', 'role' => 'user' }], model: 'openai/gpt-4o'))

```
