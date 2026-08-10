---
id: fixture_wasm_smoke_speech_mp3_format
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateSpeechRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateSpeechRequest = (() => { const _u0 = WasmCreateSpeechRequest.default(); _u0.input = "The quick brown fox jumps over the lazy dog."; _u0.model = "tts-1-hd"; _u0.responseFormat = "mp3"; _u0.speed = 1.0; _u0.voice = "nova"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.speech(request);
}

void main();

```
