---
id: fixture_java_proxy_rerank
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
        var requestJson = "{\"documents\":[\"Deep learning is a subset of machine learning using neural networks.\",\"The stock market closed higher today.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is deep learning?\"}";
var request = JsonUtil.fromJson(requestJson, RerankRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/proxy_rerank";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.rerank(request);
    }
}

```
