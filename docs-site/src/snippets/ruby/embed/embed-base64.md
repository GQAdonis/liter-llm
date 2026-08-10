---
id: fixture_ruby_embed_base64
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(encoding_format: 'base64', input: 'Test input', model: 'text-embedding-3-small'))

```
