---
id: fixture_ruby_local_chat_ollama
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 10, messages: [{ 'content' => 'Say hello in one word.', 'role' => 'user' }], model: 'ollama/qwen2:0.5b'))

```
