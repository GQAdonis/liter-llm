---
id: fixture_node_proxy_rerank
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
  const result = await client.rerank({ documents: ["Deep learning is a subset of machine learning using neural networks.", "The stock market closed higher today."], model: "rerank-v3.5", query: "What is deep learning?" });
}

void main();

```
