---
id: fixture_wasm_smoke_transcribe_basic
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateTranscriptionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateTranscriptionRequest = (() => { const _u0 = WasmCreateTranscriptionRequest.default(); _u0.file = "audio.mp3"; _u0.model = "whisper-1"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.transcribe(request);
}

void main();

```
