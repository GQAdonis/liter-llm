---
id: fixture_csharp_error_rerank_bad_request
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
  "/fixtures/error_rerank_bad_request"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.RerankAsync(new RerankRequest { Documents = new List<string> { "doc1" }, Model = "nonexistent-rerank", Query = "test" });

```
