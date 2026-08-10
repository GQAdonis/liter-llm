---
id: fixture_elixir_smoke_speech_mp3_format
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_speech_async("{\"input\":\"The quick brown fox jumps over the lazy dog.\",\"model\":\"tts-1-hd\",\"response_format\":\"mp3\",\"speed\":1.0,\"voice\":\"nova\"}")

```
