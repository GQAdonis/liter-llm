---
id: fixture_ruby_smoke_image_multiple
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-2', n: 3, prompt: 'A red bicycle', size: '256x256'))

```
