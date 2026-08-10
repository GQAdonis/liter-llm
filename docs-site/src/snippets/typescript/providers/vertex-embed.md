---
id: fixture_node_vertex_embed
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
  const result = await client.embed({ input: "Hello", model: "vertex_ai/text-embedding-005" });
}

void main();

```
