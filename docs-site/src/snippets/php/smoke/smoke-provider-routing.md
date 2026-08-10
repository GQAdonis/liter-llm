---
id: fixture_php_smoke_provider_routing
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 5, "messages" => [["content" => "Say hi.", "role" => "user"]], "model" => "openai/gpt-4o-mini"]));
$result = LiterLlm::chat($request);

```
