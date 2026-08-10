---
id: fixture_ruby_smoke_provider_routing
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 5, messages: [{ 'content' => 'Say hi.', 'role' => 'user' }], model: 'openai/gpt-4o-mini'))

```
