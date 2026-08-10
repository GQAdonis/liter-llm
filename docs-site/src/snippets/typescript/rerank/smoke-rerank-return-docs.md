---
id: fixture_node_smoke_rerank_return_docs
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
  const result = await client.rerank({ documents: ["Rust is a systems programming language.", "Iron rusts when exposed to water."], model: "rerank-v3.5", query: "What is Rust?", returnDocuments: true });
}

void main();

```
