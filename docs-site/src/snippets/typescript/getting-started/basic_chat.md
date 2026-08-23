---
id: readme_node_basic_chat
language: typescript
target: node
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

```typescript
import { createClient } from "@xberg-io/liter-llm";

const client = createClient(process.env.OPENAI_API_KEY!);
const response = await client.chat({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(response.choices[0].message.content);
```
