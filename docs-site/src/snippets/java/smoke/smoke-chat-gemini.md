---
id: fixture_java_smoke_chat_gemini
language: java
target: java
level: typecheck
requires: []
side_effect: safe
---

```java title="Java"
import io.xberg.literllm.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var requestJson = "{\"max_tokens\":10,\"messages\":[{\"content\":\"Say hello in exactly one word.\",\"role\":\"user\"}],\"model\":\"gemini/gemini-2.5-flash-lite\"}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_chat_gemini";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
