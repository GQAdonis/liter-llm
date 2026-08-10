---
id: fixture_node_single_tool_call
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
  const result = await client.chat({ messages: [{ content: "What is the weather in San Francisco?", role: "user" }], model: "gpt-4", toolChoice: "auto", tools: [{ function: { description: "Get the current weather for a given location", name: "get_weather", parameters: { properties: { location: { description: "The city and state, e.g. San Francisco, CA", type: "string" }, unit: { description: "The temperature unit to use", enum: ["celsius", "fahrenheit"], type: "string" } }, required: ["location"], type: "object" } }, type: "function" }] });
}

void main();

```
