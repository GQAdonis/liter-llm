---
id: fixture_node_proxy_moderation
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
  const result = await client.moderate({ input: "The weather is nice today.", model: "omni-moderation-latest" });
}

void main();

```
