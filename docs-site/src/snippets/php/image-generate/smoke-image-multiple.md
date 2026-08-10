---
id: fixture_php_smoke_image_multiple
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
$result = LiterLlm::imageGenerate(new \Liter\Llm\CreateImageRequest(prompt: "A red bicycle",
model: "dall-e-2",
n: 3,
size: "256x256"));

```
