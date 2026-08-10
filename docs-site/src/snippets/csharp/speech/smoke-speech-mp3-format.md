---
id: fixture_csharp_smoke_speech_mp3_format
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System;
using LiterLlm;

var baseUrl = (Environment.GetEnvironmentVariable("MOCK_SERVER_URL") ?? string.Empty) +
  "/fixtures/smoke_speech_mp3_format"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.SpeechAsync(new CreateSpeechRequest { Input = "The quick brown fox jumps over the lazy dog.", Model = "tts-1-hd", ResponseFormat = "mp3", Speed = 1.0d, Voice = "nova" });

```
