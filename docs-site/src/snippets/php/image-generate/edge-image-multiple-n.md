---
id: fixture_php_edge_image_multiple_n
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
$result = LiterLlm::imageGenerate(new \Liter\Llm\CreateImageRequest(prompt: "A cat",
model: "dall-e-3",
n: 3));

```
