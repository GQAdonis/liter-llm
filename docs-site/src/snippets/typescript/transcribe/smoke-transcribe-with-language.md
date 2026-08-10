---
id: fixture_node_smoke_transcribe_with_language
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
  const result = await client.transcribe({ file: "audio_de.mp3", language: "de", model: "whisper-1" });
}

void main();

```
