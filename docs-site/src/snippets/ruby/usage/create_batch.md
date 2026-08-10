---
id: legacy_ruby_usage_create_batch
language: ruby
target: ruby
level: syntax
requires: []
side_effect: network
---

<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require 'liter_llm'

client = LiterLlm.create_client(ENV.fetch('OPENAI_API_KEY'))

result = client.create_batch_async(
  LiterLlm::CreateBatchRequest.new(
    input_file_id: 'file-abc123',
    endpoint: '/v1/chat/completions',
    completion_window: '24h'
  )
)

puts "Batch ID: #{result.id}"
puts "Status: #{result.status}"
```
