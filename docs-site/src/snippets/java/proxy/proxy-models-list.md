---
id: fixture_java_proxy_models_list
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
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/proxy_models_list";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.listModels();
    }
}

```
