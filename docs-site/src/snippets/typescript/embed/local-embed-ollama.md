---
id: fixture_node_local_embed_ollama
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
  const result = await client.embed({ input: "The quick brown fox jumps over the lazy dog", model: "ollama/all-minilm" });
}

void main();

```
