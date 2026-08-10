---
id: fixture_node_local_stream_ollama
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
  const result = await client.chatStream({ messages: [{ content: "Count to 3", role: "user" }], model: "ollama/qwen2:0.5b", stream: true });
}

void main();

```
