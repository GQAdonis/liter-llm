---
id: fixture_php_bedrock_chat
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 16, "messages" => [["content" => "Say hello in one word.", "role" => "user"]], "model" => "bedrock/anthropic.claude-3-sonnet-20240229-v1:0", "temperature" => 0]));
$result = LiterLlm::chat($request);

```
