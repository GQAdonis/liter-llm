---
id: fixture_node_edge_rerank_single_doc
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
  const result = await client.rerank({ documents: ["Artificial intelligence is the simulation of human intelligence."], model: "rerank-v3.5", query: "What is AI?" });
}

void main();

```
