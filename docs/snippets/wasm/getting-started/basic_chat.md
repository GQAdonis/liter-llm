```typescript
import init, { LlmClient } from "@kreuzberg/liter-llm-wasm";

await init();

const client = new LlmClient({ apiKey: "sk-..." });
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});

console.log(response.choices[0].message.content);
```
