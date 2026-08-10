---
id: fixture_php_response_format_json_object
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Respond with JSON only.", "role" => "system"], ["content" => "Give me a user object with name and age fields.", "role" => "user"]], "model" => "gpt-4", "responseFormat" => ["type" => "json_object"]]));
$result = LiterLlm::chat($request);

```
