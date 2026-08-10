---
id: fixture_ruby_response_format_json_schema
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'What is the temperature in Paris today?', 'role' => 'user' }], model: 'gpt-4', response_format: { 'json_schema' => { 'name' => 'weather', 'schema' => { 'properties' => { 'temp' => { 'type' => 'number' } }, 'required' => ['temp'], 'type' => 'object' } }, 'type' => 'json_schema' }))

```
