---
id: fixture_php_local_list_models_ollama
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use Liter\Llm\LiterLlm;
$result = LiterLlm::listModels(["model" => "ollama/any"]);

```
