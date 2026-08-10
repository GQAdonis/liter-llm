---
id: fixture_php_local_provider_vllm
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Hello", "role" => "user"]], "model" => "vllm/meta-llama/Llama-3.2-1B"]));
$result = LiterLlm::chat($request);

```
