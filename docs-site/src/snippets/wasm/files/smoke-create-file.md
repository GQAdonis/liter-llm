---
id: fixture_wasm_smoke_create_file
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateFileRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateFileRequest = (() => { const _u0 = WasmCreateFileRequest.default(); _u0.file = "eyJwcm9tcHQiOiAiaGVsbG8ifQo="; _u0.filename = "training_data.jsonl"; _u0.purpose = "fine-tune"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.createFile(request);
}

void main();

```
