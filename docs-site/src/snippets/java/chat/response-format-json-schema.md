---
id: fixture_java_response_format_json_schema
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
        var requestJson = "{\"messages\":[{\"content\":\"What is the temperature in Paris today?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"json_schema\":{\"name\":\"weather\",\"schema\":{\"properties\":{\"temp\":{\"type\":\"number\"}},\"required\":[\"temp\"],\"type\":\"object\"}},\"type\":\"json_schema\"}}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/response_format_json_schema";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
