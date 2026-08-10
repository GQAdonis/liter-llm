---
id: fixture_java_seed_parameter
language: java
target: java
level: typecheck
requires: []
side_effect: network
---

```java title="Java"
import io.xberg.literllm.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var requestJson = "{\"messages\":[{\"content\":\"Pick a random number\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"seed\":42}";
var request = JsonUtil.fromJson(requestJson, ChatCompletionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/seed_parameter";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.chat(request);
    }
}

```
