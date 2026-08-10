---
id: fixture_node_all_message_types
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { createClient } from "@xberg-io/liter-llm";
async function main() {
  const client = createClient("your-api-key");
  const result = await client.chat({ messages: [{ content: "You are a helpful assistant.", role: "system" }, { content: "What is the weather in Paris?", role: "user" }, { content: null, role: "assistant", toolCalls: [{ function: { arguments: "{\"location\": \"Paris, France\"}", name: "get_weather" }, id: "call_xyz789", type: "function" }] }, { content: "{\"temperature\": 18, \"unit\": \"celsius\", \"description\": \"Partly cloudy\"}", role: "tool", toolCallId: "call_xyz789" }], model: "gpt-4" });
}

void main();

```
