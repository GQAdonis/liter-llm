---
id: fixture_node_smoke_image_basic
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
  const result = await client.imageGenerate({ model: "dall-e-3", n: 1, prompt: "A white cat sitting on a windowsill", size: "1024x1024" });
}

void main();

```
