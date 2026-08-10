---
id: fixture_ruby_batch_embed
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: ['Hello', 'World'], model: 'text-embedding-3-small'))

```
