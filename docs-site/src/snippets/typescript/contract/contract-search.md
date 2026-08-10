---
id: fixture_node_contract_search
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
  const result = await client.search({ model: "brave/web-search", query: "contract test query" });
}

void main();

```
