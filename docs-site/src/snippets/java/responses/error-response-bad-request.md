---
id: fixture_java_error_response_bad_request
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
        var requestJson = "{\"input\":\"Hello\",\"model\":\"nonexistent-model\"}";
var request = JsonUtil.fromJson(requestJson, CreateResponseRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/error_response_bad_request";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.createResponse(request);
    }
}

```
