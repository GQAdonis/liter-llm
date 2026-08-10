---
id: fixture_ruby_smoke_create_file
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_file(LiterLlm::CreateFileRequest.new(file: 'eyJwcm9tcHQiOiAiaGVsbG8ifQo=', filename: 'training_data.jsonl', purpose: 'fine-tune'))

```
