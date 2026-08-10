---
id: fixture_java_error_batch_invalid_file
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
        var requestJson = "{\"completion_window\":\"24h\",\"endpoint\":\"/v1/chat/completions\",\"input_file_id\":\"file-wrong-purpose\"}";
var request = JsonUtil.fromJson(requestJson, CreateBatchRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/error_batch_invalid_file";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.createBatch(request);
    }
}

```
