---
id: fixture_php_seed_parameter
language: php
target: php
level: typecheck
requires: []
side_effect: network
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\ChatCompletionRequest;
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Pick a random number", "role" => "user"]], "model" => "gpt-4", "seed" => 42]));
$result = LiterLlm::chat($request);

```
