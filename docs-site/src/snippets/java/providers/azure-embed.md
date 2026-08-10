---
id: fixture_java_azure_embed
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
        var requestJson = "{\"input\":\"Hello world\",\"model\":\"azure/text-embedding-ada-002\"}";
var request = JsonUtil.fromJson(requestJson, EmbeddingRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/azure_embed";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.embed(request);
    }
}

```
