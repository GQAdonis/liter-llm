---
id: fixture_node_stream_done_signal
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
  const result = await client.chatStream({ messages: [{ content: "Say done", role: "user" }], model: "gpt-4", stream: true });
}

void main();

```
