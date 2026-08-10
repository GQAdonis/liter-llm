---
id: fixture_ruby_local_embed_ollama
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.embed(LiterLlm::EmbeddingRequest.new(input: 'The quick brown fox jumps over the lazy dog', model: 'ollama/all-minilm'))

```
