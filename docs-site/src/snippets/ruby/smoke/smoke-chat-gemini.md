---
id: fixture_ruby_smoke_chat_gemini
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 10, messages: [{ 'content' => 'Say hello in exactly one word.', 'role' => 'user' }], model: 'gemini/gemini-2.5-flash-lite'))

```
