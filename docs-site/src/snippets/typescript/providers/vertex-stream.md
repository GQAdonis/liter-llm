---
id: fixture_node_vertex_stream
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
  const result = await client.chatStream({ maxTokens: 32, messages: [{ content: "Count to three, one word per response.", role: "user" }], model: "vertex_ai/gemini-2.0-flash", stream: true });
}

void main();

```
