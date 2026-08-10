---
id: fixture_node_smoke_response_with_tools
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
  const result = await client.createResponse({ input: "What is the weather in San Francisco?", model: "gpt-4o", tools: [{ description: "Get current weather for a location", name: "get_weather", parameters: { properties: { location: { type: "string" } }, required: ["location"], type: "object" }, type: "function" }] });
}

void main();

```
