---
id: fixture_node_multi_turn_conversation
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
  const result = await client.chat({ messages: [{ content: "You are a helpful assistant.", role: "system" }, { content: "What is 2 + 2?", role: "user" }, { content: "2 + 2 equals 4.", role: "assistant" }, { content: "And what is 4 + 4?", role: "user" }], model: "gpt-4" });
}

void main();

```
