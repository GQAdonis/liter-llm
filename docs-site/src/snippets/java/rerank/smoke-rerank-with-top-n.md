---
id: fixture_java_smoke_rerank_with_top_n
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
        var requestJson = "{\"documents\":[\"Python is a programming language.\",\"Cats are cute animals.\",\"Python was created by Guido van Rossum.\",\"The sun is a star.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Python?\",\"top_n\":2}";
var request = JsonUtil.fromJson(requestJson, RerankRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_rerank_with_top_n";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.rerank(request);
    }
}

```
