---
id: fixture_node_multimodal_content
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
  const result = await client.chat({ maxTokens: 100, messages: [{ content: [{ text: "What is in this image?", type: "text" }, { imageUrl: { detail: "low", url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png" }, type: "image_url" }], role: "user" }], model: "gpt-4o" });
}

void main();

```
