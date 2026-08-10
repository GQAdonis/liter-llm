---
id: fixture_ruby_edge_transcribe_empty_audio
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.transcribe(LiterLlm::CreateTranscriptionRequest.new(file: 'silence.mp3', model: 'whisper-1'))

```
