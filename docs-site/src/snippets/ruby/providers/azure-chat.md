---
id: fixture_ruby_azure_chat
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 16, messages: [{ 'content' => 'Say hello', 'role' => 'user' }], model: 'azure/gpt-4', temperature: 0))

```
