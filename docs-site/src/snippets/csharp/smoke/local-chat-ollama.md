---
id: fixture_csharp_local_chat_ollama
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
  "/fixtures/local_chat_ollama"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.ChatAsync(new ChatCompletionRequest { MaxTokens = 10, Messages = new List<string> { "{\"content\":\"Say hello in one word.\",\"role\":\"user\"}" }, Model = "ollama/qwen2:0.5b" });

```
