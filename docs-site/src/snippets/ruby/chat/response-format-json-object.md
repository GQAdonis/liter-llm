---
id: fixture_ruby_response_format_json_object
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Respond with JSON only.', 'role' => 'system' }, { 'content' => 'Give me a user object with name and age fields.', 'role' => 'user' }], model: 'gpt-4', response_format: { 'type' => 'json_object' }))

```
