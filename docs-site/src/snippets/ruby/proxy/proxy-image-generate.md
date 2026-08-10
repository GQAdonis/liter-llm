---
id: fixture_ruby_proxy_image_generate
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-3', n: 1, prompt: 'A sunset over the ocean', size: '1024x1024'))

```
