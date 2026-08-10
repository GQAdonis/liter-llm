---
id: fixture_ruby_single_tool_call
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'What is the weather in San Francisco?', 'role' => 'user' }], model: 'gpt-4', tool_choice: 'auto', tools: [{ 'function' => { 'description' => 'Get the current weather for a given location', 'name' => 'get_weather', 'parameters' => { 'properties' => { 'location' => { 'description' => 'The city and state, e.g. San Francisco, CA', 'type' => 'string' }, 'unit' => { 'description' => 'The temperature unit to use', 'enum' => ['celsius', 'fahrenheit'], 'type' => 'string' } }, 'required' => ['location'], 'type' => 'object' } }, 'type' => 'function' }]))

```
