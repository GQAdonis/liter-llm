---
id: fixture_elixir_smoke_transcribe_basic
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_transcribe_async("{\"file\":\"audio.mp3\",\"model\":\"whisper-1\"}")

```
