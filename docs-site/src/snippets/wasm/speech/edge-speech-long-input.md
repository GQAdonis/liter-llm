---
id: fixture_wasm_edge_speech_long_input
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmCreateSpeechRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmCreateSpeechRequest = (() => { const _u0 = WasmCreateSpeechRequest.default(); _u0.input = "This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. End of input."; _u0.model = "tts-1"; _u0.voice = "echo"; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.speech(request);
}

void main();

```
