---
id: fixture_ruby_proxy_chat_basic
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Say hello', 'role' => 'user' }], model: 'openai/gpt-4o'))

```
