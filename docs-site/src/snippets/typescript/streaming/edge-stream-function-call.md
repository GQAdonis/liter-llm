---
id: fixture_node_edge_stream_function_call
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
  const result = await client.chatStream({ messages: [{ content: "What's the weather?", role: "user" }], model: "gpt-4", tools: [{ function: { name: "get_weather", parameters: { properties: { city: { type: "string" } }, type: "object" } }, type: "function" }] });
}

void main();

```
