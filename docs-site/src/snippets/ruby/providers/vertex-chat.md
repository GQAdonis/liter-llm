---
id: fixture_ruby_vertex_chat
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 16, messages: [{ 'content' => 'Say hello in one word.', 'role' => 'user' }], model: 'vertex_ai/gemini-2.0-flash', temperature: 0))

```
