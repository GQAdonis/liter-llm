---
id: fixture_ruby_error_transcribe_bad_format
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.transcribe(LiterLlm::CreateTranscriptionRequest.new(file: 'audio.xyz', model: 'whisper-1'))

```
