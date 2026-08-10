---
id: fixture_csharp_smoke_rerank_basic
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
  "/fixtures/smoke_rerank_basic"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.RerankAsync(new RerankRequest { Documents = new List<string> { "Machine learning is a subset of AI.", "The weather is sunny today.", "Deep learning uses neural networks." }, Model = "rerank-v3.5", Query = "What is machine learning?" });

```
