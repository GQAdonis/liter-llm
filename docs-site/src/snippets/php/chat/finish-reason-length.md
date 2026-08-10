---
id: fixture_php_finish_reason_length
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 5, "messages" => [["content" => "Tell me a long story", "role" => "user"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
