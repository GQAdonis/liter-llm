---
id: fixture_wasm_tool_choice_specific
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmChatCompletionRequest, createClient } from "@xberg-io/liter-llm-wasm";
async function main() {
  const request: WasmChatCompletionRequest = (() => { const _u0 = WasmChatCompletionRequest.default(); _u0.messages = [{ content: "What is the weather in Paris?", role: "user" }]; _u0.model = "gpt-4"; _u0.toolChoice = { function: { name: "get_weather" }, type: "function" }; _u0.tools = [(() => { const _u1 = WasmChatCompletionTool.default(); _u1.function = (() => { const _u2 = WasmFunctionDefinition.default(); _u2.description = "Get the current weather for a given location"; _u2.name = "get_weather"; _u2.parameters = { properties: { location: { description: "The city name", type: "string" } }, required: ["location"], type: "object" }; return _u2; })(); _u1.type = "function"; return _u1; })(), (() => { const _u1 = WasmChatCompletionTool.default(); _u1.function = (() => { const _u2 = WasmFunctionDefinition.default(); _u2.description = "Search the web for information"; _u2.name = "search_web"; _u2.parameters = { properties: { query: { description: "The search query", type: "string" } }, required: ["query"], type: "object" }; return _u2; })(); _u1.type = "function"; return _u1; })()]; return _u0; })();
  const client = createClient("your-api-key");
  const result = await client.chat(request);
}

void main();

```
