---
id: fixture_node_tool_choice_required
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
  const result = await client.chat({ messages: [{ content: "What is the weather today?", role: "user" }], model: "gpt-4", toolChoice: "required", tools: [{ function: { description: "Get the current weather for a given location", name: "get_weather", parameters: { properties: { location: { description: "The city name", type: "string" } }, required: ["location"], type: "object" } }, type: "function" }] });
}

void main();

```
