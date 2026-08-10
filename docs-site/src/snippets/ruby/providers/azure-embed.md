---
id: fixture_ruby_azure_embed
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: 'Hello world', model: 'azure/text-embedding-ada-002'))

```
