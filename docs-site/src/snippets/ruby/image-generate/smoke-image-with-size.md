---
id: fixture_ruby_smoke_image_with_size
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-3', n: 1, prompt: 'A sunset over mountains', size: '1792x1024'))

```
