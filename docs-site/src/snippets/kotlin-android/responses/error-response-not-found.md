---
id: fixture_kotlin_android_error_response_not_found
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.literllm.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = LiterLlm.retrieveResponse("resp-nonexistent")
}

```
