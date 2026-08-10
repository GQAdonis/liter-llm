---
id: fixture_php_edge_chat_system_only
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "You are a helpful and concise assistant", "role" => "system"], ["content" => "Hi", "role" => "user"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
