---
id: fixture_csharp_smoke_rerank_with_top_n
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
  "/fixtures/smoke_rerank_with_top_n"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.RerankAsync(new RerankRequest { Documents = new List<string> { "Python is a programming language.", "Cats are cute animals.", "Python was created by Guido van Rossum.", "The sun is a star." }, Model = "rerank-v3.5", Query = "What is Python?", TopN = 2 });

```
