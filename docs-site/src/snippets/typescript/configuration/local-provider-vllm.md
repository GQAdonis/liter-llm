---
id: fixture_node_local_provider_vllm
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
  const result = await client.chat({ messages: [{ content: "Hello", role: "user" }], model: "vllm/meta-llama/Llama-3.2-1B" });
}

void main();

```
