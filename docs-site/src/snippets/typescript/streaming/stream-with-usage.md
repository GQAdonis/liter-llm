---
id: fixture_node_stream_with_usage
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
  const result = await client.chatStream({ messages: [{ content: "Say hi", role: "user" }], model: "gpt-4", stream: true, streamOptions: { includeUsage: true } });
}

void main();

```
