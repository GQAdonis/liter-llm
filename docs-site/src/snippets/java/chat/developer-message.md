---
id: fixture_java_developer_message
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
        var requestJson = "{\"messages\":[{\"content\":\"You are a coding assistant. Always respond with concise code examples.\",\"role\":\"developer\"},{\"content\":\"How do I reverse a string in Python?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/developer_message";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
