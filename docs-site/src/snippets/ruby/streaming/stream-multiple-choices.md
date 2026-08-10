---
id: fixture_ruby_stream_multiple_choices
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Hello', 'role' => 'user' }], model: 'gpt-4o', n: 2, stream: true)).to_a

```
