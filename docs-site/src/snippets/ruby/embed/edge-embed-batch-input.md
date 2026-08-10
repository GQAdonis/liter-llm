---
id: fixture_ruby_edge_embed_batch_input
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: ['Hello world', 'Goodbye world'], model: 'text-embedding-3-small'))

```
