---
id: fixture_ruby_proxy_embeddings
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: 'Hello world', model: 'openai/text-embedding-3-small'))

```
