---
id: readme_csharp_basic_chat
language: csharp
target: csharp
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

```csharp
using LiterLlm;

using var client = LiterLlmConverter.CreateClient(
    apiKey: Environment.GetEnvironmentVariable("OPENAI_API_KEY")!,
    baseUrl: null, timeoutSecs: null, maxRetries: null, modelHint: null);

var response = await client.ChatAsync(new ChatCompletionRequest
{
    Model = "openai/gpt-4o",
    Messages = [new Message.User(new UserMessage { Content = UserContent.Of("Hello!") })]
});
Console.WriteLine(response.Choices[0].Message.Content);
```
