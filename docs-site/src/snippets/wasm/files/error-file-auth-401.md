---
id: fixture_wasm_error_file_auth_401
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
  try {
    await client.listFiles(undefined);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
