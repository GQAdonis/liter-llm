---
id: fixture_node_search_with_max_results
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
  const result = await client.search({ maxResults: 2, model: "brave/web-search", query: "Rust programming" });
}

void main();

```
