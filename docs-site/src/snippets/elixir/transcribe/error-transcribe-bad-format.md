---
id: fixture_elixir_error_transcribe_bad_format
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_transcribe_async("{\"file\":\"audio.xyz\",\"model\":\"whisper-1\"}")

```
