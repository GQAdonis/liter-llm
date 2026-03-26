```java
import dev.kreuzberg.literlm.LlmClient;
import dev.kreuzberg.literlm.ChatRequest;
import dev.kreuzberg.literlm.Message;

public class Main {
    public static void main(String[] args) throws Exception {
        try (var client = new LlmClient()) {
            var response = client.chat(ChatRequest.builder()
                .model("openai/gpt-4o")
                .message(new Message("user", "Hello!"))
                .build());
            System.out.println(response.getContent());
        }
    }
}
```
