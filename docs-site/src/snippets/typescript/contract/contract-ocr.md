---
id: fixture_node_contract_ocr
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
  const result = await client.ocr({ document: { type: "document_url", url: "https://example.com/contract-test.pdf" }, model: "mistral/mistral-ocr-latest" });
}

void main();

```
