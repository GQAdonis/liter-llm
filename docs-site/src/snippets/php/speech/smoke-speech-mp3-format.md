---
id: fixture_php_smoke_speech_mp3_format
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
$result = LiterLlm::speech(new \Liter\Llm\CreateSpeechRequest(model: "tts-1-hd",
input: "The quick brown fox jumps over the lazy dog.",
voice: "nova",
responseFormat: "mp3",
speed: 1.0));

```
