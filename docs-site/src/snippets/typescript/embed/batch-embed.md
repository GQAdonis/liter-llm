---
id: fixture_node_batch_embed
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
  const result = await client.embed({ input: ["Hello", "World"], model: "text-embedding-3-small" });
}

void main();

```
