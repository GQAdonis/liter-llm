---
id: fixture_java_edge_file_large_upload
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
        var requestJson = "{\"file\":\"eyJwcm9tcHQiOiAibGFyZ2UgdHJhaW5pbmcgZGF0YSJ9Cg==\",\"filename\":\"large_training_data.jsonl\",\"purpose\":\"fine-tune\"}";
var request = JsonUtil.fromJson(requestJson, CreateFileRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/edge_file_large_upload";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.createFile(request);
    }
}

```
