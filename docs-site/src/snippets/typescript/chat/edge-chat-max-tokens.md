---
id: fixture_node_edge_chat_max_tokens
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
  const result = await client.chat({ maxTokens: 1, messages: [{ content: "Write a story", role: "user" }], model: "gpt-4" });
}

void main();

```
