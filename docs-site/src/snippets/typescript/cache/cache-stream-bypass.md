---
id: fixture_node_cache_stream_bypass
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
  const result = await client.chatStream({ messages: [{ content: "Hello", role: "user" }], model: "gpt-4" });
}

void main();

```
