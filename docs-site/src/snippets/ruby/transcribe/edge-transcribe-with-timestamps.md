---
id: fixture_ruby_edge_transcribe_with_timestamps
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.transcribe(LiterLlm::CreateTranscriptionRequest.new(file: 'audio.mp3', model: 'whisper-1', response_format: 'verbose_json'))

```
