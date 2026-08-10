---
id: fixture_node_response_format_json_object
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
  const result = await client.chat({ messages: [{ content: "Respond with JSON only.", role: "system" }, { content: "Give me a user object with name and age fields.", role: "user" }], model: "gpt-4", responseFormat: { type: "json_object" } });
}

void main();

```
