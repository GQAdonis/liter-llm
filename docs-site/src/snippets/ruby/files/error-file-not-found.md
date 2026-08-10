---
id: fixture_ruby_error_file_not_found
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.retrieve_file('file-nonexistent')

```
