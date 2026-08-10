---
id: fixture_php_multimodal_content
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
$request = \Liter\Llm\ChatCompletionRequest::from_json(json_encode(["maxTokens" => 100, "messages" => [["content" => [["text" => "What is in this image?", "type" => "text"], ["imageUrl" => ["detail" => "low", "url" => "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/280px-PNG_transparency_demonstration_1.png"], "type" => "image_url"]], "role" => "user"]], "model" => "gpt-4o"]));
$result = LiterLlm::chat($request);

```
