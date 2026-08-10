---
id: fixture_kotlin_android_seed_parameter
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: network
---

```kotlin title="Kotlin (Android)"
import io.xberg.literllm.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = LiterLlm.chat(request)
}

```
