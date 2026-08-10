---
id: fixture_ruby_stream_with_tool_calls
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
chunks = LiterLlm.chat_stream(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'What is the weather in NYC?', 'role' => 'user' }], model: 'gpt-4', stream: true, tools: [{ 'function' => { 'description' => 'Get the current weather for a given location', 'name' => 'get_weather', 'parameters' => { 'properties' => { 'location' => { 'description' => 'The city and state, e.g. New York, NY', 'type' => 'string' } }, 'required' => ['location'], 'type' => 'object' } }, 'type' => 'function' }])).to_a

```
