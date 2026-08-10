---
id: fixture_node_ocr_error_401
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
  try {
    await client.ocr({ document: { type: "document_url", url: "https://example.com/doc.pdf" }, model: "mistral/mistral-ocr-latest" });
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
