---
id: fixture_node_anthropic_chat
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
  const result = await client.chat({ maxTokens: 16, messages: [{ content: "You are a helpful assistant.", role: "system" }, { content: "Say hello in one word.", role: "user" }], model: "anthropic/claude-3-5-sonnet-20241022", temperature: 0 });
}

void main();

```
