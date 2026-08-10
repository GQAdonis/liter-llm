---
id: fixture_php_error_speech_bad_model
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateSpeechRequest;
try {
    LiterLlm::speech(new \Liter\Llm\CreateSpeechRequest(model: "tts-nonexistent",
input: "Hello",
voice: "alloy"));
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
