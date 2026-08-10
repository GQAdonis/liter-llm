---
id: fixture_java_anthropic_stream
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
        var requestJson = "{\"max_tokens\":32,\"messages\":[{\"content\":\"Count to three, one word per response.\",\"role\":\"user\"}],\"model\":\"anthropic/claude-3-5-sonnet-20241022\",\"stream\":true}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/anthropic_stream";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chatStream(request);
    }
}

```
