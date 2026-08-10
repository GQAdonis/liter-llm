---
id: fixture_ruby_error_file_bad_purpose
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_file(LiterLlm::CreateFileRequest.new(file: 'data.jsonl', purpose: 'invalid-purpose'))

```
