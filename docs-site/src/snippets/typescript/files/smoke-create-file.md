---
id: fixture_node_smoke_create_file
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
  const result = await client.createFile({ file: "eyJwcm9tcHQiOiAiaGVsbG8ifQo=", filename: "training_data.jsonl", purpose: "fine-tune" });
}

void main();

```
