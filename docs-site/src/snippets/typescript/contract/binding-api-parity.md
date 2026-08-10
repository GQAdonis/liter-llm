---
id: fixture_node_binding_api_parity
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
  const result = await client.chat({ messages: [{ content: "Contract test", role: "user" }], model: "openai/gpt-4o" });
}

void main();

```
