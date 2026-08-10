---
id: fixture_php_edge_image_b64_response
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
use Liter\Llm\CreateImageRequest;
use Liter\Llm\Image;
$result = LiterLlm::imageGenerate(new \Liter\Llm\CreateImageRequest(prompt: "A blue circle",
model: "dall-e-3",
n: 1,
size: "1024x1024",
responseFormat: "b64_json"));

```
