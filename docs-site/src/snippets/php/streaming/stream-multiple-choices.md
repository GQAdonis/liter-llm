---
id: fixture_php_stream_multiple_choices
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Hello", "role" => "user"]], "model" => "gpt-4o", "n" => 2, "stream" => true]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
