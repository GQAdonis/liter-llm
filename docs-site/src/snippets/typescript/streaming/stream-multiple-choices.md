---
id: fixture_node_stream_multiple_choices
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
  const result = await client.chatStream({ messages: [{ content: "Hello", role: "user" }], model: "gpt-4o", n: 2, stream: true });
}

void main();

```
