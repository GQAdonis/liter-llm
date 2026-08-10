---
id: fixture_java_edge_speech_all_voices
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
        var requestJson = "{\"input\":\"Hello world\",\"model\":\"tts-1\",\"voice\":\"nova\"}";
var request = JsonUtil.fromJson(requestJson, CreateSpeechRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/edge_speech_all_voices";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.speech(request);
    }
}

```
