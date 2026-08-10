---
id: fixture_elixir_edge_transcribe_with_timestamps
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_transcribe_async("{\"file\":\"audio.mp3\",\"model\":\"whisper-1\",\"response_format\":\"verbose_json\"}")

```
