---
id: fixture_ruby_smoke_speech_basic
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.speech(LiterLlm::CreateSpeechRequest.new(input: 'Hello, world!', model: 'tts-1', voice: 'alloy'))

```
