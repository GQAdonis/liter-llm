---
id: fixture_php_error_transcribe_auth_401
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateTranscriptionRequest;
try {
    LiterLlm::transcribe(new \Liter\Llm\CreateTranscriptionRequest(model: "whisper-1",
file: "audio.mp3"));
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
