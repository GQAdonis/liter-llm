---
id: fixture_php_edge_transcribe_empty_audio
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
$result = LiterLlm::transcribe(new \Liter\Llm\CreateTranscriptionRequest(model: "whisper-1",
file: "silence.mp3"));

```
