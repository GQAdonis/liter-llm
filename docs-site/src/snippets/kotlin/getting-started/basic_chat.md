---
id: readme_kotlin_basic_chat
language: kotlin
target: kotlin
level: syntax
requires: []
side_effect: network
---

Send a message to any provider using the `provider/model` prefix.

```kotlin
import io.xberg.literllm.android.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val client = LiterLlm.createClient(System.getenv("OPENAI_API_KEY") ?: "")
    val request = ChatCompletionRequest(
        model = "openai/gpt-4o",
        messages = listOf(Message.User(UserMessage(content = UserContent.Text("Hello!"))))
    )
    val response = client.chat(request)
    println(response.choices[0].message.content)
}
```
