---
id: fixture_node_azure_stream
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
  const result = await client.chatStream({ messages: [{ content: "Count to 3", role: "user" }], model: "azure/gpt-4", stream: true, temperature: 0 });
}

void main();

```
