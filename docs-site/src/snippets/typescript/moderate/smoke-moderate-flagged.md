---
id: fixture_node_smoke_moderate_flagged
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
  const result = await client.moderate({ input: "I want to hurt someone very badly", model: "omni-moderation-latest" });
}

void main();

```
