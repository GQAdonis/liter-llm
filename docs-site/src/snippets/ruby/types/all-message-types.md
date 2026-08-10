---
id: fixture_ruby_all_message_types
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'You are a helpful assistant.', 'role' => 'system' }, { 'content' => 'What is the weather in Paris?', 'role' => 'user' }, { 'content' => nil, 'role' => 'assistant', 'tool_calls' => [{ 'function' => { 'arguments' => '{"location": "Paris, France"}', 'name' => 'get_weather' }, 'id' => 'call_xyz789', 'type' => 'function' }] }, { 'content' => '{"temperature": 18, "unit": "celsius", "description": "Partly cloudy"}', 'role' => 'tool', 'tool_call_id' => 'call_xyz789' }], model: 'gpt-4'))

```
