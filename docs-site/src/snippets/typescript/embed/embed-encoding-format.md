---
id: fixture_node_embed_encoding_format
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
  const result = await client.embed({ encodingFormat: "float", input: "Test input", model: "text-embedding-3-small" });
}

void main();

```
