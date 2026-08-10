---
id: fixture_ruby_edge_image_multiple_n
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-3', n: 3, prompt: 'A cat'))

```
