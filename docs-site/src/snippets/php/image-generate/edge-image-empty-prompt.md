---
id: fixture_php_edge_image_empty_prompt
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
try {
    LiterLlm::imageGenerate(new \Liter\Llm\CreateImageRequest(prompt: "",
model: "dall-e-3",
n: 1,
size: "1024x1024"));
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
