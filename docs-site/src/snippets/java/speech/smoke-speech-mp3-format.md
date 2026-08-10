---
id: fixture_java_smoke_speech_mp3_format
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
        var requestJson = "{\"input\":\"The quick brown fox jumps over the lazy dog.\",\"model\":\"tts-1-hd\",\"response_format\":\"mp3\",\"speed\":1.0,\"voice\":\"nova\"}";
var request = JsonUtil.fromJson(requestJson, CreateSpeechRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_speech_mp3_format";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.speech(request);
    }
}

```
