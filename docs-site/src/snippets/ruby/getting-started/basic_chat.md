---
id: legacy_ruby_getting_started_basic_chat
language: ruby
target: ruby
level: syntax
requires: []
side_effect: network
---

```ruby
# frozen_string_literal: true

require 'liter_llm'

client = LiterLlm.create_client(ENV.fetch('OPENAI_API_KEY'))

result = client.chat_async(
  LiterLlm::ChatCompletionRequest.new(
    model: 'openai/gpt-4o-mini',
    messages: [{ 'role' => 'user', 'content' => 'Hello!' }]
  )
)

puts result.choices[0].message.content
```
