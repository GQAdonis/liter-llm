---
id: fixture_ruby_seed_parameter
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: network
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(messages: [{ 'content' => 'Pick a random number', 'role' => 'user' }], model: 'gpt-4', seed: 42))

```
