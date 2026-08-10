---
id: fixture_elixir_error_speech_bad_model
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_speech_async("{\"input\":\"Hello\",\"model\":\"tts-nonexistent\",\"voice\":\"alloy\"}")

```
