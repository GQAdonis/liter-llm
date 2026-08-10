---
id: fixture_node_azure_chat
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
  const result = await client.chat({ maxTokens: 16, messages: [{ content: "Say hello", role: "user" }], model: "azure/gpt-4", temperature: 0 });
}

void main();

```
