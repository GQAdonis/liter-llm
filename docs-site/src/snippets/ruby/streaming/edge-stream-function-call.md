---
id: fixture_ruby_edge_stream_function_call
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => "What's the weather?", 'role' => 'user' }], model: 'gpt-4', tools: [{ 'function' => { 'name' => 'get_weather', 'parameters' => { 'properties' => { 'city' => { 'type' => 'string' } }, 'type' => 'object' } }, 'type' => 'function' }])).to_a

```
