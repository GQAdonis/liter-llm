---
id: fixture_java_all_message_types
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
        var requestJson = "{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"},{\"content\":null,\"role\":\"assistant\",\"tool_calls\":[{\"function\":{\"arguments\":\"{\\\"location\\\": \\\"Paris, France\\\"}\",\"name\":\"get_weather\"},\"id\":\"call_xyz789\",\"type\":\"function\"}]},{\"content\":\"{\\\"temperature\\\": 18, \\\"unit\\\": \\\"celsius\\\", \\\"description\\\": \\\"Partly cloudy\\\"}\",\"role\":\"tool\",\"tool_call_id\":\"call_xyz789\"}],\"model\":\"gpt-4\"}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/all_message_types";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
