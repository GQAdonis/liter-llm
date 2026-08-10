---
id: fixture_php_smoke_streaming_openai
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 50, "messages" => [["content" => "Count from 1 to 5.", "role" => "user"]], "model" => "openai/gpt-4o-mini"]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
