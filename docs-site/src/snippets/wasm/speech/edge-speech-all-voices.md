---
id: fixture_wasm_edge_speech_all_voices
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateSpeechRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateSpeechRequest = (() => { const _u0 = WasmCreateSpeechRequest.default(); _u0.input = "Hello world"; _u0.model = "tts-1"; _u0.voice = "nova"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.speech(request);
}

void main();

```
