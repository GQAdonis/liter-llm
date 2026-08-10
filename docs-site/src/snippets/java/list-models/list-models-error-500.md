---
id: fixture_java_list_models_error_500
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
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/list_models_error_500";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.listModels();
    }
}

```
