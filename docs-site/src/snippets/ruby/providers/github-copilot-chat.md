---
id: fixture_ruby_github_copilot_chat
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 16, messages: [{ 'content' => 'Say hello in one word.', 'role' => 'user' }], model: 'github_copilot/gpt-4o', temperature: 0))

```
