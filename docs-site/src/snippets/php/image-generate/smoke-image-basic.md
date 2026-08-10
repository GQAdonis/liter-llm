---
id: fixture_php_smoke_image_basic
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
$result = LiterLlm::imageGenerate(new \Liter\Llm\CreateImageRequest(prompt: "A white cat sitting on a windowsill",
model: "dall-e-3",
n: 1,
size: "1024x1024"));

```
