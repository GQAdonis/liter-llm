---
id: fixture_ruby_edge_chat_max_tokens
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 1, messages: [{ 'content' => 'Write a story', 'role' => 'user' }], model: 'gpt-4'))

```
