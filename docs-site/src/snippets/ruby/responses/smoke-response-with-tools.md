---
id: fixture_ruby_smoke_response_with_tools
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_response(LiterLlm::CreateResponseRequest.new(input: 'What is the weather in San Francisco?', model: 'gpt-4o', tools: [{ 'description' => 'Get current weather for a location', 'name' => 'get_weather', 'parameters' => { 'properties' => { 'location' => { 'type' => 'string' } }, 'required' => ['location'], 'type' => 'object' }, 'type' => 'function' }]))

```
