---
id: fixture_node_local_chat_ollama
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
  const result = await client.chat({ maxTokens: 10, messages: [{ content: "Say hello in one word.", role: "user" }], model: "ollama/qwen2:0.5b" });
}

void main();

```
