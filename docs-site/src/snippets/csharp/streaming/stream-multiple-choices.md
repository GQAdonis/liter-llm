---
id: fixture_csharp_stream_multiple_choices
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
  "/fixtures/stream_multiple_choices"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatStreamAsync(new ChatCompletionRequest { Messages = new List<string> { "{\"content\":\"Hello\",\"role\":\"user\"}" }, Model = "gpt-4o", N = 2, Stream = true });

```
