---
id: fixture_wasm_smoke_list_models_openai
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
  const result = await client.listModels();
}

void main();

```
