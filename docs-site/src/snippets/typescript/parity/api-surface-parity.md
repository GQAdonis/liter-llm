---
id: fixture_node_api_surface_parity
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
  const result = await client.chat({ messages: [{ content: "Hello", role: "user" }], model: "gpt-4o" });
}

void main();

```
