---
id: fixture_csharp_edge_speech_long_input
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
  "/fixtures/edge_speech_long_input"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.SpeechAsync(new CreateSpeechRequest { Input = "This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. End of input.", Model = "tts-1", Voice = "echo" });

```
