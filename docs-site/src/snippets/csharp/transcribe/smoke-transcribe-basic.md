---
id: fixture_csharp_smoke_transcribe_basic
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
  "/fixtures/smoke_transcribe_basic"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.TranscribeAsync(new CreateTranscriptionRequest { File = "audio.mp3", Model = "whisper-1" });

```
