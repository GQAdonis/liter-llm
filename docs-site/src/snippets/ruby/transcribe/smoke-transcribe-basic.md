---
id: fixture_ruby_smoke_transcribe_basic
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
