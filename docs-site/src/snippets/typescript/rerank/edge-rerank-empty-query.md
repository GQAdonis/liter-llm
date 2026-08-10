---
id: fixture_node_edge_rerank_empty_query
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
  const result = await client.rerank({ documents: ["Some document", "Another document"], model: "rerank-v3.5", query: "" });
}

void main();

```
