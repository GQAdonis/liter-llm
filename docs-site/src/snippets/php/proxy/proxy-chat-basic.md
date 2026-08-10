---
id: fixture_php_proxy_chat_basic
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Say hello", "role" => "user"]], "model" => "openai/gpt-4o"]));
$result = LiterLlm::chat($request);

```
