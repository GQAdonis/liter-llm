---
id: fixture_java_smoke_speech_basic
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
        var requestJson = "{\"input\":\"Hello, world!\",\"model\":\"tts-1\",\"voice\":\"alloy\"}";
var request = JsonUtil.fromJson(requestJson, CreateSpeechRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_speech_basic";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.speech(request);
    }
}

```
