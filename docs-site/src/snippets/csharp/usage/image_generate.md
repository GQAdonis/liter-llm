---
id: legacy_csharp_usage_image_generate
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

var response = await client.ImageGenerate(new CreateImageRequest
{
    Model = "openai/dall-e-3",
    Prompt = "A sunset over mountains",
    N = 1,
    Size = "1024x1024"
});
Console.WriteLine(response.Data[0].Url);
```
