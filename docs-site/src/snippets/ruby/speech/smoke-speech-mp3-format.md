---
id: fixture_ruby_smoke_speech_mp3_format
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.speech(LiterLlm::CreateSpeechRequest.new(input: 'The quick brown fox jumps over the lazy dog.', model: 'tts-1-hd', response_format: 'mp3', speed: 1.0, voice: 'nova'))

```
