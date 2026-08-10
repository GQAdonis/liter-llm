---
id: fixture_node_smoke_provider_routing
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
  const result = await client.chat({ maxTokens: 5, messages: [{ content: "Say hi.", role: "user" }], model: "openai/gpt-4o-mini" });
}

void main();

```
