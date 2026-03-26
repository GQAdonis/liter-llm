```java
import dev.kreuzberg.literlm.LlmClient;
import dev.kreuzberg.literlm.ChatRequest;
import dev.kreuzberg.literlm.Message;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = new LlmClient()) {
            client.chatStream(
                ChatRequest.builder()
                    .model("openai/gpt-4o")
                    .message(new Message("user", "Tell me a story"))
                    .build(),
                chunk -> System.out.print(chunk.getDelta())
            );
            System.out.println();
        }
    }
}
```
