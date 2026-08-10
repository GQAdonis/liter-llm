---
id: fixture_java_smoke_rerank_basic
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
        var requestJson = "{\"documents\":[\"Machine learning is a subset of AI.\",\"The weather is sunny today.\",\"Deep learning uses neural networks.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is machine learning?\"}";
var request = JsonUtil.fromJson(requestJson, RerankRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_rerank_basic";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.rerank(request);
    }
}

```
