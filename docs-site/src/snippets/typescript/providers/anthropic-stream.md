---
id: fixture_node_anthropic_stream
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
  const result = await client.chatStream({ maxTokens: 32, messages: [{ content: "Count to three, one word per response.", role: "user" }], model: "anthropic/claude-3-5-sonnet-20241022", stream: true });
}

void main();

```
