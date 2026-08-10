---
id: fixture_java_smoke_create_response
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
        var requestJson = "{\"input\":\"Explain quantum computing in one sentence.\",\"model\":\"gpt-4o\"}";
var request = JsonUtil.fromJson(requestJson, CreateResponseRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_create_response";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.createResponse(request);
    }
}

```
