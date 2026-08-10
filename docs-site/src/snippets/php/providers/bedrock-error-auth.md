---
id: fixture_php_bedrock_error_auth
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Hello", "role" => "user"]], "model" => "bedrock/anthropic.claude-3-sonnet-20240229-v1:0"]));
try {
    LiterLlm::chat($request);
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
