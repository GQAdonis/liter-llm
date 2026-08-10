---
id: fixture_node_edge_speech_long_input
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
  const result = await client.speech({ input: "This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. End of input.", model: "tts-1", voice: "echo" });
}

void main();

```
