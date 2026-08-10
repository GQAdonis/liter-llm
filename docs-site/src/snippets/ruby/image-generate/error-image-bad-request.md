---
id: fixture_ruby_error_image_bad_request
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.image_generate(LiterLlm::CreateImageRequest.new(model: 'dall-e-3', n: 1, prompt: 'A cat', size: '9999x9999'))

```
