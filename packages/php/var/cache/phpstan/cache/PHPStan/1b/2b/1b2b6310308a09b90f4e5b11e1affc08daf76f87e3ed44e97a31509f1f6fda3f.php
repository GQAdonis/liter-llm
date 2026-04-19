<?php declare(strict_types = 1);

// osfsl-/Users/naamanhirschfeld/workspace/kreuzberg-dev/liter-llm/packages/php/stubs/liter_llm_extension.php-PHPStan\BetterReflection\Reflection\ReflectionClass-Liter\Llm\DefaultClient
return \PHPStan\Cache\CacheItem::__set_state(array(
   'variableKey' => 'v2-2bdafbd6b337dee2441af7db0d29f0a46acf71417691c212879329d5062c0635-8.4.20-6.70.0.0',
   'data' => 
  array (
    'locatedSource' => 
    array (
      'class' => 'PHPStan\\BetterReflection\\SourceLocator\\Located\\LocatedSource',
      'data' => 
      array (
        'name' => 'Liter\\Llm\\DefaultClient',
        'filename' => '/Users/naamanhirschfeld/workspace/kreuzberg-dev/liter-llm/packages/php/stubs/liter_llm_extension.php',
      ),
    ),
    'namespace' => 'Liter\\Llm',
    'name' => 'Liter\\Llm\\DefaultClient',
    'shortName' => 'DefaultClient',
    'isInterface' => false,
    'isTrait' => false,
    'isEnum' => false,
    'isBackedEnum' => false,
    'modifiers' => 0,
    'docComment' => '/**
 * Default client implementation backed by `reqwest`.
 *
 * The provider is resolved at construction time from `model_hint` (or
 * defaults to OpenAI).  However, individual requests can override the
 * provider when their model string contains a prefix that clearly
 * identifies a different provider (e.g. `"anthropic/claude-3"` will
 * route to Anthropic even if the client was built without a hint).
 *
 * When the model prefix does not match any known provider, the
 * construction-time provider is used as the fallback.
 *
 * The provider is stored behind an [`Arc`] so it can be shared cheaply into
 * async closures and streaming tasks that must be `\'static`.
 */',
    'attributes' => 
    array (
    ),
    'startLine' => 35,
    'endLine' => 37,
    'startColumn' => 1,
    'endColumn' => 1,
    'parentClassName' => NULL,
    'implementsClassNames' => 
    array (
    ),
    'traitClassNames' => 
    array (
    ),
    'immediateConstants' => 
    array (
    ),
    'immediateProperties' => 
    array (
    ),
    'immediateMethods' => 
    array (
    ),
    'traitsData' => 
    array (
      'aliases' => 
      array (
      ),
      'modifiers' => 
      array (
      ),
      'precedences' => 
      array (
      ),
      'hashes' => 
      array (
      ),
    ),
  ),
));