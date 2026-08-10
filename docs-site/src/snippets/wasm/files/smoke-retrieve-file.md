---
id: fixture_wasm_smoke_retrieve_file
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const client = createClient("your-api-key");
  const result = await client.retrieveFile("file-abc123");
}

void main();

```
