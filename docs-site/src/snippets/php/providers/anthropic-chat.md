---
id: fixture_php_anthropic_chat
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 16, "messages" => [["content" => "You are a helpful assistant.", "role" => "system"], ["content" => "Say hello in one word.", "role" => "user"]], "model" => "anthropic/claude-3-5-sonnet-20241022", "temperature" => 0]));
$result = LiterLlm::chat($request);

```
