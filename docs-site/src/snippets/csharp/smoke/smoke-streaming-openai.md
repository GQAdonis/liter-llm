---
id: fixture_csharp_smoke_streaming_openai
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
  "/fixtures/smoke_streaming_openai"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatStreamAsync(new ChatCompletionRequest { MaxTokens = 50, Messages = new List<string> { "{\"content\":\"Count from 1 to 5.\",\"role\":\"user\"}" }, Model = "openai/gpt-4o-mini" });

```
