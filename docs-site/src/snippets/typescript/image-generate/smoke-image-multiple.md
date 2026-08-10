---
id: fixture_node_smoke_image_multiple
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
  const result = await client.imageGenerate({ model: "dall-e-2", n: 3, prompt: "A red bicycle", size: "256x256" });
}

void main();

```
