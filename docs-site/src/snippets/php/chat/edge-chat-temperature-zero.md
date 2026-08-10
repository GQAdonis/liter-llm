---
id: fixture_php_edge_chat_temperature_zero
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Say hello", "role" => "user"]], "model" => "gpt-4", "temperature" => 0]));
$result = LiterLlm::chat($request);

```
