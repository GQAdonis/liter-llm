---
id: fixture_node_edge_chat_system_only
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
  const result = await client.chat({ messages: [{ content: "You are a helpful and concise assistant", role: "system" }, { content: "Hi", role: "user" }], model: "gpt-4" });
}

void main();

```
