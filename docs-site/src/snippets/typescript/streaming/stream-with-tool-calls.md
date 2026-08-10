---
id: fixture_node_stream_with_tool_calls
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
  const result = await client.chatStream({ messages: [{ content: "What is the weather in NYC?", role: "user" }], model: "gpt-4", stream: true, tools: [{ function: { description: "Get the current weather for a given location", name: "get_weather", parameters: { properties: { location: { description: "The city and state, e.g. New York, NY", type: "string" } }, required: ["location"], type: "object" } }, type: "function" }] });
}

void main();

```
