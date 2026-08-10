---
id: fixture_csharp_error_speech_auth_401
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
  "/fixtures/error_speech_auth_401"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.SpeechAsync(new CreateSpeechRequest { Input = "Hello", Model = "tts-1", Voice = "alloy" });

```
