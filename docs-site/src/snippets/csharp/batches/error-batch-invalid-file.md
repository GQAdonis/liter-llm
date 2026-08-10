---
id: fixture_csharp_error_batch_invalid_file
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
  "/fixtures/error_batch_invalid_file"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.CreateBatchAsync(new CreateBatchRequest { CompletionWindow = "24h", Endpoint = "/v1/chat/completions", InputFileId = "file-wrong-purpose" });

```
