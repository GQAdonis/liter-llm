---
id: fixture_ruby_error_transcribe_auth_401
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.transcribe(LiterLlm::CreateTranscriptionRequest.new(file: 'audio.mp3', model: 'whisper-1'))

```
