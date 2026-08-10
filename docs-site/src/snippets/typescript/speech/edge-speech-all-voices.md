---
id: fixture_node_edge_speech_all_voices
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
  const result = await client.speech({ input: "Hello world", model: "tts-1", voice: "nova" });
}

void main();

```
