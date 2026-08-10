---
id: fixture_csharp_stream_with_usage
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
  "/fixtures/stream_with_usage"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatStreamAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"Say hi\",\"role\":\"user\"}" }, Model = "gpt-4", Stream = true, StreamOptions = new StreamOptions { IncludeUsage = true } });

```
