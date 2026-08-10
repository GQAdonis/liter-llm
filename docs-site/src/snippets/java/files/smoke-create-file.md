---
id: fixture_java_smoke_create_file
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
        var requestJson = "{\"file\":\"eyJwcm9tcHQiOiAiaGVsbG8ifQo=\",\"filename\":\"training_data.jsonl\",\"purpose\":\"fine-tune\"}";
var request = JsonUtil.fromJson(requestJson, CreateFileRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_create_file";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.createFile(request);
    }
}

```
