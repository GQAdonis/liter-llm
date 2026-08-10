---
id: fixture_java_edge_stream_function_call
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
        var requestJson = "{\"messages\":[{\"content\":\"What's the weather?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tools\":[{\"function\":{\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"city\":{\"type\":\"string\"}},\"type\":\"object\"}},\"type\":\"function\"}]}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/edge_stream_function_call";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chatStream(request);
    }
}

```
