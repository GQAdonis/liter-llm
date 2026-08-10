---
id: fixture_csharp_smoke_create_batch
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
  "/fixtures/smoke_create_batch"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateBatchAsync(new CreateBatchRequest { CompletionWindow = "24h", Endpoint = "/v1/chat/completions", InputFileId = "file-abc123" });

```
