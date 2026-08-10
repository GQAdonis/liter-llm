---
id: fixture_node_edge_chat_temperature_zero
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
  const result = await client.chat({ messages: [{ content: "Say hello", role: "user" }], model: "gpt-4", temperature: 0 });
}

void main();

```
