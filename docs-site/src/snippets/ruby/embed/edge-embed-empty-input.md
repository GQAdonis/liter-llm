---
id: fixture_ruby_edge_embed_empty_input
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: '', model: 'text-embedding-3-small'))

```
