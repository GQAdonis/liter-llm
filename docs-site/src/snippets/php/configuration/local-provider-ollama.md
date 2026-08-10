---
id: fixture_php_local_provider_ollama
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Hello", "role" => "user"]], "model" => "ollama/qwen2:0.5b"]));
$result = LiterLlm::chat($request);

```
