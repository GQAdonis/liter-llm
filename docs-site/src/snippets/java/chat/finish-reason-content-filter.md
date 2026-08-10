---
id: fixture_java_finish_reason_content_filter
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
        var requestJson = "{\"messages\":[{\"content\":\"Tell me something controversial\",\"role\":\"user\"}],\"model\":\"gpt-4\"}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/finish_reason_content_filter";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
