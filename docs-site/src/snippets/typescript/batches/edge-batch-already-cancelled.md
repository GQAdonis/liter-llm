---
id: fixture_node_edge_batch_already_cancelled
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
  try {
    await client.cancelBatch("batch-cancelled001");
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
