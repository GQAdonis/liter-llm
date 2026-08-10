---
id: fixture_ruby_smoke_transcribe_with_language
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.transcribe(LiterLlm::CreateTranscriptionRequest.new(file: 'audio_de.mp3', language: 'de', model: 'whisper-1'))

```
