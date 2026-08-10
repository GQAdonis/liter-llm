---
id: fixture_php_multi_turn_conversation
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "You are a helpful assistant.", "role" => "system"], ["content" => "What is 2 + 2?", "role" => "user"], ["content" => "2 + 2 equals 4.", "role" => "assistant"], ["content" => "And what is 4 + 4?", "role" => "user"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
