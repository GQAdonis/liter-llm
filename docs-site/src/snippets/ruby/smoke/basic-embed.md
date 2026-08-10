---
id: fixture_ruby_basic_embed
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: 'Hello world', model: 'text-embedding-3-small'))

```
