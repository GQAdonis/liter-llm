---
id: fixture_node_smoke_image_with_size
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
  const result = await client.imageGenerate({ model: "dall-e-3", n: 1, prompt: "A sunset over mountains", size: "1792x1024" });
}

void main();

```
