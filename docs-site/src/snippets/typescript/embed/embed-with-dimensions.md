---
id: fixture_node_embed_with_dimensions
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
  const result = await client.embed({ dimensions: 256, input: "Hello world", model: "text-embedding-3-small" });
}

void main();

```
