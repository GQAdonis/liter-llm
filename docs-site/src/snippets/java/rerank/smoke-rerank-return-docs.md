---
id: fixture_java_smoke_rerank_return_docs
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
        var requestJson = "{\"documents\":[\"Rust is a systems programming language.\",\"Iron rusts when exposed to water.\"],\"model\":\"rerank-v3.5\",\"query\":\"What is Rust?\",\"return_documents\":true}";
var request = JsonUtil.fromJson(requestJson, RerankRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_rerank_return_docs";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.rerank(request);
    }
}

```
