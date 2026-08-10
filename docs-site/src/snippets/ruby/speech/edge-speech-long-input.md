---
id: fixture_ruby_edge_speech_long_input
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "liter_llm"
result = LiterLlm.speech(LiterLlm::CreateSpeechRequest.new(input: 'This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. End of input.', model: 'tts-1', voice: 'echo'))

```
