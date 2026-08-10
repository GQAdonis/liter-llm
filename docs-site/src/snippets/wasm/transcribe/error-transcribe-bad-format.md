---
id: fixture_wasm_error_transcribe_bad_format
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateTranscriptionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateTranscriptionRequest = (() => { const _u0 = WasmCreateTranscriptionRequest.default(); _u0.file = "audio.xyz"; _u0.model = "whisper-1"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.transcribe(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
