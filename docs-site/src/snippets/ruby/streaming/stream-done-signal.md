---
id: fixture_ruby_stream_done_signal
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Say done', 'role' => 'user' }], model: 'gpt-4', stream: true)).to_a

```
