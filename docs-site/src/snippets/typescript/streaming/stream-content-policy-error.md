---
id: fixture_node_stream_content_policy_error
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
    await client.chatStream({ messages: [{ content: "Generate harmful content", role: "user" }], model: "gpt-4o", stream: true });
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
