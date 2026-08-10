---
id: fixture_php_edge_stream_function_call
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "What's the weather?", "role" => "user"]], "model" => "gpt-4", "tools" => [["function" => ["name" => "get_weather", "parameters" => ["properties" => ["city" => ["type" => "string"]], "type" => "object"]], "type" => "function"]]]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
