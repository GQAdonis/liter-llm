---
id: fixture_java_edge_embed_empty_input
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
        var requestJson = "{\"input\":\"\",\"model\":\"text-embedding-3-small\"}";
var request = JsonUtil.fromJson(requestJson, EmbeddingRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/edge_embed_empty_input";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.embed(request);
    }
}

```
