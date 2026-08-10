---
id: fixture_elixir_smoke_transcribe_with_language
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_transcribe_async("{\"file\":\"audio_de.mp3\",\"language\":\"de\",\"model\":\"whisper-1\"}")

```
