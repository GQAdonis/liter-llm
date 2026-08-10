---
id: fixture_node_smoke_create_response
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
  const result = await client.createResponse({ input: "Explain quantum computing in one sentence.", model: "gpt-4o" });
}

void main();

```
