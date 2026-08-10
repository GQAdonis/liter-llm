---
id: fixture_ruby_embed_encoding_format
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(encoding_format: 'float', input: 'Test input', model: 'text-embedding-3-small'))

```
