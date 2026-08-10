---
id: fixture_php_developer_message
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "You are a coding assistant. Always respond with concise code examples.", "role" => "developer"], ["content" => "How do I reverse a string in Python?", "role" => "user"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
