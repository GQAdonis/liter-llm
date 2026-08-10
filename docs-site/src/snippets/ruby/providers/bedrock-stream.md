---
id: fixture_ruby_bedrock_stream
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(max_tokens: 32, messages: [{ 'content' => 'Count to three, one word per response.', 'role' => 'user' }], model: 'bedrock/anthropic.claude-3-sonnet-20240229-v1:0', stream: true)).to_a

```
