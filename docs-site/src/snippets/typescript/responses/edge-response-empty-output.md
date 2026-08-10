---
id: fixture_node_edge_response_empty_output
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
  const result = await client.createResponse({ input: "", model: "gpt-4o" });
}

void main();

```
