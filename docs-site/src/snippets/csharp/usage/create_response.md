---
id: legacy_csharp_usage_create_response
language: csharp
target: csharp
level: syntax
requires: []
side_effect: network
---

<!-- snippet:compile-only -->

```csharp
using LiterLlm;

using var client = LiterLlmLib.CreateClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
    baseUrl: null, timeoutSecs: null, maxRetries: null, modelHint: null);

var response = await client.CreateResponse(new CreateResponseRequest
{
    Model = "openai/gpt-4o",
    Input = "Explain quantum computing in one sentence."
});
Console.WriteLine(response);
```
