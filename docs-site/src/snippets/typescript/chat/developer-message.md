---
id: fixture_node_developer_message
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
  const result = await client.chat({ messages: [{ content: "You are a coding assistant. Always respond with concise code examples.", role: "developer" }, { content: "How do I reverse a string in Python?", role: "user" }], model: "gpt-4" });
}

void main();

```
