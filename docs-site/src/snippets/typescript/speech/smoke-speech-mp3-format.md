---
id: fixture_node_smoke_speech_mp3_format
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
  const result = await client.speech({ input: "The quick brown fox jumps over the lazy dog.", model: "tts-1-hd", responseFormat: "mp3", speed: 1.0, voice: "nova" });
}

void main();

```
