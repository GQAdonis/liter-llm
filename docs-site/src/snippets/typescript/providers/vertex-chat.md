---
id: fixture_node_vertex_chat
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
  const result = await client.chat({ maxTokens: 16, messages: [{ content: "Say hello in one word.", role: "user" }], model: "vertex_ai/gemini-2.0-flash", temperature: 0 });
}

void main();

```
