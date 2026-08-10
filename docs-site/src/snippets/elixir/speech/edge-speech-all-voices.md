---
id: fixture_elixir_edge_speech_all_voices
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = LiterLlm.defaultclient_speech_async("{\"input\":\"Hello world\",\"model\":\"tts-1\",\"voice\":\"nova\"}")

```
