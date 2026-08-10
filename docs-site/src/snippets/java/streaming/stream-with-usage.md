---
id: fixture_java_stream_with_usage
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
        var requestJson = "{\"messages\":[{\"content\":\"Say hi\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"stream\":true,\"stream_options\":{\"include_usage\":true}}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/stream_with_usage";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chatStream(request);
    }
}

```
