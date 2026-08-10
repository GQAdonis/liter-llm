---
id: fixture_ruby_smoke_streaming_openai
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(max_tokens: 50, messages: [{ 'content' => 'Count from 1 to 5.', 'role' => 'user' }], model: 'openai/gpt-4o-mini')).to_a

```
