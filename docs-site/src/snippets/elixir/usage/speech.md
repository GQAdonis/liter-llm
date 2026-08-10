---
id: legacy_elixir_usage_speech
language: elixir
target: elixir
level: syntax
requires: []
side_effect: network
---

<!-- snippet:compile-only -->

```elixir
{:ok, client} = LiterLlm.create_client(System.get_env("OPENAI_API_KEY"))

request =
  Jason.encode!(%{
    model: "openai/tts-1",
    input: "Hello, world!",
    voice: "alloy"
  })

{:ok, audio_bytes} = LiterLlm.defaultclient_speech_async(client, request)
File.write!("output.mp3", audio_bytes)
IO.puts("Wrote #{byte_size(audio_bytes)} bytes to output.mp3")
```
