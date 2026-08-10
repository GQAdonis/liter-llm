---
id: fixture_java_smoke_delete_file
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
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_delete_file";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.deleteFile("file-abc123");
    }
}

```
