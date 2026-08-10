---
id: fixture_csharp_edge_speech_all_voices
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
  "/fixtures/edge_speech_all_voices"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.SpeechAsync(new CreateSpeechRequest { Input = "Hello world", Model = "tts-1", Voice = "nova" });

```
