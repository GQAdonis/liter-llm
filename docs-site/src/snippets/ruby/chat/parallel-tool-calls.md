---
id: fixture_ruby_parallel_tool_calls
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'What is the weather in NYC and London?', 'role' => 'user' }], model: 'gpt-4', parallel_tool_calls: true, tools: [{ 'function' => { 'description' => 'Get the current weather for a given location', 'name' => 'get_weather', 'parameters' => { 'properties' => { 'location' => { 'description' => 'The city name', 'type' => 'string' } }, 'required' => ['location'], 'type' => 'object' } }, 'type' => 'function' }]))

```
