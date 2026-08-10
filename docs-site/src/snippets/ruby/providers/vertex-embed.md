---
id: fixture_ruby_vertex_embed
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: 'Hello', model: 'vertex_ai/text-embedding-005'))

```
