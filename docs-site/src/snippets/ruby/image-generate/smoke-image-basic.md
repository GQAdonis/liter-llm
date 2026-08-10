---
id: fixture_ruby_smoke_image_basic
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-3', n: 1, prompt: 'A white cat sitting on a windowsill', size: '1024x1024'))

```
