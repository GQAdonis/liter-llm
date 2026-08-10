---
id: fixture_ruby_anthropic_tool_calling
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 256, messages: [{ 'content' => 'What is the weather in London?', 'role' => 'user' }], model: 'anthropic/claude-3-5-sonnet-20241022', tool_choice: 'auto', tools: [{ 'function' => { 'description' => 'Get the current weather for a given location', 'name' => 'get_weather', 'parameters' => { 'properties' => { 'location' => { 'description' => 'The city and country, e.g. London, UK', 'type' => 'string' }, 'unit' => { 'description' => 'The temperature unit to use', 'enum' => ['celsius', 'fahrenheit'], 'type' => 'string' } }, 'required' => ['location'], 'type' => 'object' } }, 'type' => 'function' }]))

```
