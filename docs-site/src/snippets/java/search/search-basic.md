---
id: fixture_java_search_basic
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
        var requestJson = "{\"model\":\"brave/web-search\",\"query\":\"What is Rust programming language?\"}";
var request = JsonUtil.fromJson(requestJson, SearchRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/search_basic";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.search(request);
    }
}

```
