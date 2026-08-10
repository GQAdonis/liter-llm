---
id: fixture_java_error_file_bad_purpose
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
        var requestJson = "{\"file\":\"data.jsonl\",\"purpose\":\"invalid-purpose\"}";
var request = JsonUtil.fromJson(requestJson, CreateFileRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/error_file_bad_purpose";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.createFile(request);
    }
}

```
