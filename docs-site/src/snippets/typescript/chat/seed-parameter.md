---
id: fixture_node_seed_parameter
language: typescript
target: node
level: typecheck
requires: []
side_effect: network
---

```typescript title="TypeScript"
import { createClient } from "@xberg-io/liter-llm";
async function main() {
  const client = createClient("your-api-key");
  const result = await client.chat({ messages: [{ content: "Pick a random number", role: "user" }], model: "gpt-4", seed: 42 });
}

void main();

```
