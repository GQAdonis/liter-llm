---
id: fixture_csharp_proxy_rerank
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
  "/fixtures/proxy_rerank"; var client = LiterLlmConverter.CreateClient("test-key", baseUrl, null, null,
  null);
var result = await client.RerankAsync(new RerankRequest { Documents = new List<string> { "Deep learning is a subset of machine learning using neural networks.", "The stock market closed higher today." }, Model = "rerank-v3.5", Query = "What is deep learning?" });

```
