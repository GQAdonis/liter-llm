---
id: fixture_csharp_error_transcribe_bad_format
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
  "/fixtures/error_transcribe_bad_format"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.TranscribeAsync(new CreateTranscriptionRequest { File = "audio.xyz", Model = "whisper-1" });

```
