---
id: fixture_java_proxy_chat_basic
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
        var requestJson = "{\"messages\":[{\"content\":\"Say hello\",\"role\":\"user\"}],\"model\":\"openai/gpt-4o\"}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/proxy_chat_basic";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
