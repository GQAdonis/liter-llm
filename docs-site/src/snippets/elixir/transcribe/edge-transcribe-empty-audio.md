---
id: fixture_elixir_edge_transcribe_empty_audio
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_transcribe_async("{\"file\":\"silence.mp3\",\"model\":\"whisper-1\"}")

```
