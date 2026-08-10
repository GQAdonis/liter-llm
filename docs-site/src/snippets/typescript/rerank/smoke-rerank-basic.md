---
id: fixture_node_smoke_rerank_basic
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
  const result = await client.rerank({ documents: ["Machine learning is a subset of AI.", "The weather is sunny today.", "Deep learning uses neural networks."], model: "rerank-v3.5", query: "What is machine learning?" });
}

void main();

```
