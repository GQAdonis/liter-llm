---
id: readme_swift_basic_chat
language: swift
target: swift
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

```swift
import Foundation
import LiterLlm

let client = try await LiterLlm.createClient(apiKey: ProcessInfo.processInfo.environment["OPENAI_API_KEY"] ?? "")
let request = ChatCompletionRequest(
    model: "openai/gpt-4o",
    messages: [.user(field0: .init(content: .text(field0: "Hello!")))],
    temperature: nil, topP: nil, maxTokens: nil, toolChoice: nil, tools: nil, responseFormat: nil
)
let response = try await client.chat(request)
print(response.choices[0].message.content ?? "")
```
