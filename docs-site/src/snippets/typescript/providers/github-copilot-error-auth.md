---
id: fixture_node_github_copilot_error_auth
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
  try {
    await client.chat({ messages: [{ content: "Hello", role: "user" }], model: "github_copilot/gpt-4o" });
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
