---
id: readme_ruby_basic_chat
language: ruby
target: ruby
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

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
