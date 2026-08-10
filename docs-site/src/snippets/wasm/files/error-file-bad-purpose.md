---
id: fixture_wasm_error_file_bad_purpose
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateFileRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateFileRequest = (() => { const _u0 = WasmCreateFileRequest.default(); _u0.file = "data.jsonl"; _u0.purpose = "invalid-purpose"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.createFile(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
