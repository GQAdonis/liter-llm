---
id: fixture_ruby_multimodal_content
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.chat(LiterLlm::ChatCompletionRequest.new(max_tokens: 100, messages: [{ 'content' => [{ 'text' => 'What is in this image?', 'type' => 'text' }, { 'image_url' => { 'detail' => 'low', 'url' => 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png' }, 'type' => 'image_url' }], 'role' => 'user' }], model: 'gpt-4o'))

```
