---
id: fixture_ruby_cache_stream_bypass
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Hello', 'role' => 'user' }], model: 'gpt-4')).to_a

```
