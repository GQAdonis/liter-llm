---
id: fixture_ruby_error_batch_invalid_file
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.create_batch(LiterLlm::CreateBatchRequest.new(completion_window: '24h', endpoint: '/v1/chat/completions', input_file_id: 'file-wrong-purpose'))

```
