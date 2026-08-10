---
id: fixture_ruby_edge_speech_all_voices
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.speech(LiterLlm::CreateSpeechRequest.new(input: 'Hello world', model: 'tts-1', voice: 'nova'))

```
