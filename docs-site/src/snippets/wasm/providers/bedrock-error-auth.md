---
id: fixture_wasm_bedrock_error_auth
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "Hello", role: "user" }]; _u0.model = "bedrock/anthropic.claude-3-sonnet-20240229-v1:0"; return _u0; })();
  const client = createClient("your-api-key");
  try {
    await client.chat(request);
  } catch (error) {
    console.error("Call failed as expected:", error);
    return;
  }
  throw new Error("expected call to fail");
}

void main();

```
