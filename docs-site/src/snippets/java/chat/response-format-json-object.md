---
id: fixture_java_response_format_json_object
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
        var requestJson = "{\"messages\":[{\"content\":\"Respond with JSON only.\",\"role\":\"system\"},{\"content\":\"Give me a user object with name and age fields.\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"type\":\"json_object\"}}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/response_format_json_object";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
