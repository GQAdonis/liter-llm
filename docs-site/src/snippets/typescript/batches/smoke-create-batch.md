---
id: fixture_node_smoke_create_batch
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
  const result = await client.createBatch({ completionWindow: "24h", endpoint: "/v1/chat/completions", inputFileId: "file-abc123" });
}

void main();

```
