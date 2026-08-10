---
id: fixture_ruby_edge_image_b64_response
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-3', n: 1, prompt: 'A blue circle', response_format: 'b64_json', size: '1024x1024'))

```
