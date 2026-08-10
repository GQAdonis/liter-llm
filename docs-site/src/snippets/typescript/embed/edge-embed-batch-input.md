---
id: fixture_node_edge_embed_batch_input
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
  const result = await client.embed({ input: ["Hello world", "Goodbye world"], model: "text-embedding-3-small" });
}

void main();

```
