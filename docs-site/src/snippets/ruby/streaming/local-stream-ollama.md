---
id: fixture_ruby_local_stream_ollama
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Count to 3', 'role' => 'user' }], model: 'ollama/qwen2:0.5b', stream: true)).to_a

```
