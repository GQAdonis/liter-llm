---
id: fixture_elixir_smoke_speech_basic
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_speech_async("{\"input\":\"Hello, world!\",\"model\":\"tts-1\",\"voice\":\"alloy\"}")

```
