---
id: fixture_ruby_stream_with_usage
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Say hi', 'role' => 'user' }], model: 'gpt-4', stream: true, stream_options: { 'include_usage' => true })).to_a

```
