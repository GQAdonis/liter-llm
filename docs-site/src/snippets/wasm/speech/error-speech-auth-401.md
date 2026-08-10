---
id: fixture_wasm_error_speech_auth_401
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateSpeechRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateSpeechRequest = (() => { const _u0 = WasmCreateSpeechRequest.default(); _u0.input = "Hello"; _u0.model = "tts-1"; _u0.voice = "alloy"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.speech(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
