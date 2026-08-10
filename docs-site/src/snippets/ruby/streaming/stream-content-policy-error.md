---
id: fixture_ruby_stream_content_policy_error
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Generate harmful content', 'role' => 'user' }], model: 'gpt-4o', stream: true)).to_a

```
