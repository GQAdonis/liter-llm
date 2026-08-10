---
id: fixture_php_finish_reason_content_filter
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Tell me something controversial", "role" => "user"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
