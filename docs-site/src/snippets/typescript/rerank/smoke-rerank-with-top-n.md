---
id: fixture_node_smoke_rerank_with_top_n
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
  const result = await client.rerank({ documents: ["Python is a programming language.", "Cats are cute animals.", "Python was created by Guido van Rossum.", "The sun is a star."], model: "rerank-v3.5", query: "What is Python?", topN: 2 });
}

void main();

```
