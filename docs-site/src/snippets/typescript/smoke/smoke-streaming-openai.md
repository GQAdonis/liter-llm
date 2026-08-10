---
id: fixture_node_smoke_streaming_openai
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
  const result = await client.chatStream({ maxTokens: 50, messages: [{ content: "Count from 1 to 5.", role: "user" }], model: "openai/gpt-4o-mini" });
}

void main();

```
