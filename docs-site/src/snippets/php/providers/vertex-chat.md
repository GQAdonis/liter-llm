---
id: fixture_php_vertex_chat
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 16, "messages" => [["content" => "Say hello in one word.", "role" => "user"]], "model" => "vertex_ai/gemini-2.0-flash", "temperature" => 0]));
$result = LiterLlm::chat($request);

```
