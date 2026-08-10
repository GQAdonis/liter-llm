---
id: fixture_java_contract_search
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
        var requestJson = "{\"model\":\"brave/web-search\",\"query\":\"contract test query\"}";
var request = JsonUtil.fromJson(requestJson, SearchRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/contract_search";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.search(request);
    }
}

```
