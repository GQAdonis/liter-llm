---
id: fixture_wasm_edge_file_large_upload
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateFileRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateFileRequest = (() => { const _u0 = WasmCreateFileRequest.default(); _u0.file = "eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg=="; _u0.filename = "large_training_data.jsonl"; _u0.purpose = "fine-tune"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.createFile(request);
}

void main();

```
