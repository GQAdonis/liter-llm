---
id: fixture_ruby_proxy_chat_streaming
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Count to 3', 'role' => 'user' }], model: 'openai/gpt-4o', stream: true)).to_a

```
