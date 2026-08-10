---
id: fixture_java_edge_rerank_empty_query
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
        var requestJson = "{\"documents\":[\"Some document\",\"Another document\"],\"model\":\"rerank-v3.5\",\"query\":\"\"}";
var request = JsonUtil.fromJson(requestJson, RerankRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/edge_rerank_empty_query";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.rerank(request);
    }
}

```
