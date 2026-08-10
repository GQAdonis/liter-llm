---
id: fixture_php_stream_with_usage
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
use Liter\Llm\Usage;
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["messages" => [["content" => "Say hi", "role" => "user"]], "model" => "gpt-4", "stream" => true, "streamOptions" => ["includeUsage" => true]]));
$chunks = iterator_to_array(LiterLlm::chatStream($request));

```
