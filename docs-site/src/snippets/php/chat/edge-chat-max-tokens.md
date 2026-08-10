---
id: fixture_php_edge_chat_max_tokens
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 1, "messages" => [["content" => "Write a story", "role" => "user"]], "model" => "gpt-4"]));
$result = LiterLlm::chat($request);

```
