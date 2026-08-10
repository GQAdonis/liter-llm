---
id: fixture_java_smoke_transcribe_basic
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
        var requestJson = "{\"file\":\"audio.mp3\",\"model\":\"whisper-1\"}";
var request = JsonUtil.fromJson(requestJson, CreateTranscriptionRequest.class);
        var baseUrl = System.getenv().getOrDefault("MOCK_SERVER_URL", "") + "/fixtures/smoke_transcribe_basic";
        var client = LiterLlm.createClient("test-key", baseUrl, null, null, null);
        var result = client.transcribe(request);
    }
}

```
