---
id: fixture_ruby_embed_with_dimensions
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(dimensions: 256, input: 'Hello world', model: 'text-embedding-3-small'))

```
