---
id: fixture_java_error_batch_not_found
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
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/error_batch_not_found";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.retrieveBatch("batch-nonexistent");
    }
}

```
