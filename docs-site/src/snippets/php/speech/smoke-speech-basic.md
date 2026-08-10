---
id: fixture_php_smoke_speech_basic
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
$result = LiterLlm::speech(new \Liter\Llm\CreateSpeechRequest(model: "tts-1",
input: "Hello, world!",
voice: "alloy"));

```
