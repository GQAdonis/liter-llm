---
id: fixture_wasm_list_models_error_500
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
    await client.listModels();
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
