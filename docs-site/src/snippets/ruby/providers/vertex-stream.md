---
id: fixture_ruby_vertex_stream
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(max_tokens: 32, messages: [{ 'content' => 'Count to three, one word per response.', 'role' => 'user' }], model: 'vertex_ai/gemini-2.0-flash', stream: true)).to_a

```
