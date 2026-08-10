---
id: fixture_node_smoke_transcribe_basic
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
  const result = await client.transcribe({ file: "audio.mp3", model: "whisper-1" });
}

void main();

```
