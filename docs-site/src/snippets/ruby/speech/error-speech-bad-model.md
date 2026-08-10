---
id: fixture_ruby_error_speech_bad_model
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.speech(LiterLlm::CreateSpeechRequest.new(input: 'Hello', model: 'tts-nonexistent', voice: 'alloy'))

```
