---
id: fixture_php_smoke_chat_gemini
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\ChatCompletionRequest;
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 10, "messages" => [["content" => "Say hello in exactly one word.", "role" => "user"]], "model" => "gemini/gemini-2.5-flash-lite"]));
$result = LiterLlm::chat($request);

```
