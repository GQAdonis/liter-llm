---
id: fixture_php_empty_stream
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Say nothing", "role" => "user"]], "model" => "gpt-4", "stream" => true]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
