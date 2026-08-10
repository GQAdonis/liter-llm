---
id: fixture_node_smoke_chat_gemini
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
  const result = await client.chat({ maxTokens: 10, messages: [{ content: "Say hello in exactly one word.", role: "user" }], model: "gemini/gemini-2.5-flash-lite" });
}

void main();

```
