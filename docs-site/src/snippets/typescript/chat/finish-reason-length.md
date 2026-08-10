---
id: fixture_node_finish_reason_length
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
  const result = await client.chat({ maxTokens: 5, messages: [{ content: "Tell me a long story", role: "user" }], model: "gpt-4" });
}

void main();

```
