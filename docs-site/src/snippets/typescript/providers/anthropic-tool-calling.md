---
id: fixture_node_anthropic_tool_calling
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
  const result = await client.chat({ maxTokens: 256, messages: [{ content: "What is the weather in London?", role: "user" }], model: "anthropic/claude-3-5-sonnet-20241022", toolChoice: "auto", tools: [{ function: { description: "Get the current weather for a given location", name: "get_weather", parameters: { properties: { location: { description: "The city and country, e.g. London, UK", type: "string" }, unit: { description: "The temperature unit to use", enum: ["celsius", "fahrenheit"], type: "string" } }, required: ["location"], type: "object" } }, type: "function" }] });
}

void main();

```
