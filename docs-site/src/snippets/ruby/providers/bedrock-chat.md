---
id: fixture_ruby_bedrock_chat
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 16, messages: [{ 'content' => 'Say hello in one word.', 'role' => 'user' }], model: 'bedrock/anthropic.claude-3-sonnet-20240229-v1:0', temperature: 0))

```
