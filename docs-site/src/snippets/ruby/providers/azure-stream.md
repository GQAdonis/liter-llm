---
id: fixture_ruby_azure_stream
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Count to 3', 'role' => 'user' }], model: 'azure/gpt-4', stream: true, temperature: 0)).to_a

```
