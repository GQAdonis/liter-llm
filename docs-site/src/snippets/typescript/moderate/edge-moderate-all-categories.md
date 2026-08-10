---
id: fixture_node_edge_moderate_all_categories
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
  const result = await client.moderate({ input: "Extremely harmful content targeting multiple categories", model: "omni-moderation-latest" });
}

void main();

```
