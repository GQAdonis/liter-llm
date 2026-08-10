---
id: fixture_node_edge_file_large_upload
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
  const result = await client.createFile({ file: "eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==", filename: "large_training_data.jsonl", purpose: "fine-tune" });
}

void main();

```
