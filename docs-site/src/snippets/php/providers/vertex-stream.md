---
id: fixture_php_vertex_stream
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 32, "messages" => [["content" => "Count to three, one word per response.", "role" => "user"]], "model" => "vertex_ai/gemini-2.0-flash", "stream" => true]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
