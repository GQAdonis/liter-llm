```ruby
# frozen_string_literal: true

require "liter_lm"

client = LiterLm::Client.new
response = client.chat(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }]
)
puts response.content
```
