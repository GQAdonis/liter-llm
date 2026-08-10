---
id: fixture_node_response_format_json_schema
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
  const result = await client.chat({ messages: [{ content: "What is the temperature in Paris today?", role: "user" }], model: "gpt-4", responseFormat: { jsonSchema: { name: "weather", schema: { properties: { temp: { type: "number" } }, required: ["temp"], type: "object" } }, type: "json_schema" } });
}

void main();

```
