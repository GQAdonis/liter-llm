---
id: fixture_ruby_smoke_cache_memory
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 5, messages: [{ 'content' => 'What is 2+2? Answer with just the number.', 'role' => 'user' }], model: 'openai/gpt-4o-mini'))

```
