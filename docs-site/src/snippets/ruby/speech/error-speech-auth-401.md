---
id: fixture_ruby_error_speech_auth_401
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.speech(LiterLlm::CreateSpeechRequest.new(input: 'Hello', model: 'tts-1', voice: 'alloy'))

```
