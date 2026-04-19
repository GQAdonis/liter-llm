<?php declare(strict_types = 1);

// odsl-/Users/naamanhirschfeld/workspace/kreuzberg-dev/liter-llm/packages/php/src/LiterLlm.php-PHPStan\BetterReflection\Reflection\ReflectionClass-Liter\Llm\LiterLlm
return \PHPStan\Cache\CacheItem::__set_state(array(
   'variableKey' => 'v2-6.70.0.0-8.4.20-3337b25ef9dc13f446dad77054744414c0bd73e9feead315ad9bddbcf5e7098a',
   'data' => 
  array (
    'locatedSource' => 
    array (
      'class' => 'PHPStan\\BetterReflection\\SourceLocator\\Located\\LocatedSource',
      'data' => 
      array (
        'name' => 'Liter\\Llm\\LiterLlm',
        'filename' => '/Users/naamanhirschfeld/workspace/kreuzberg-dev/liter-llm/packages/php/src/LiterLlm.php',
      ),
    ),
    'namespace' => 'Liter\\Llm',
    'name' => 'Liter\\Llm\\LiterLlm',
    'shortName' => 'LiterLlm',
    'isInterface' => false,
    'isTrait' => false,
    'isEnum' => false,
    'isBackedEnum' => false,
    'modifiers' => 32,
    'docComment' => NULL,
    'attributes' => 
    array (
    ),
    'startLine' => 7,
    'endLine' => 92,
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
      'createClient' => 
      array (
        'name' => 'createClient',
        'parameters' => 
        array (
          'api_key' => 
          array (
            'name' => 'api_key',
            'default' => NULL,
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
              'data' => 
              array (
                'name' => 'string',
                'isIdentifier' => true,
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 29,
            'endLine' => 29,
            'startColumn' => 41,
            'endColumn' => 55,
            'parameterIndex' => 0,
            'isOptional' => false,
          ),
          'base_url' => 
          array (
            'name' => 'base_url',
            'default' => 
            array (
              'code' => 'null',
              'attributes' => 
              array (
                'startLine' => 29,
                'endLine' => 29,
                'startTokenPos' => 46,
                'startFilePos' => 889,
                'endTokenPos' => 46,
                'endFilePos' => 892,
              ),
            ),
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionUnionType',
              'data' => 
              array (
                'types' => 
                array (
                  0 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'string',
                      'isIdentifier' => true,
                    ),
                  ),
                  1 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'null',
                      'isIdentifier' => true,
                    ),
                  ),
                ),
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 29,
            'endLine' => 29,
            'startColumn' => 58,
            'endColumn' => 81,
            'parameterIndex' => 1,
            'isOptional' => true,
          ),
          'timeout_secs' => 
          array (
            'name' => 'timeout_secs',
            'default' => 
            array (
              'code' => 'null',
              'attributes' => 
              array (
                'startLine' => 29,
                'endLine' => 29,
                'startTokenPos' => 56,
                'startFilePos' => 916,
                'endTokenPos' => 56,
                'endFilePos' => 919,
              ),
            ),
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionUnionType',
              'data' => 
              array (
                'types' => 
                array (
                  0 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'int',
                      'isIdentifier' => true,
                    ),
                  ),
                  1 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'null',
                      'isIdentifier' => true,
                    ),
                  ),
                ),
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 29,
            'endLine' => 29,
            'startColumn' => 84,
            'endColumn' => 108,
            'parameterIndex' => 2,
            'isOptional' => true,
          ),
          'max_retries' => 
          array (
            'name' => 'max_retries',
            'default' => 
            array (
              'code' => 'null',
              'attributes' => 
              array (
                'startLine' => 29,
                'endLine' => 29,
                'startTokenPos' => 66,
                'startFilePos' => 942,
                'endTokenPos' => 66,
                'endFilePos' => 945,
              ),
            ),
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionUnionType',
              'data' => 
              array (
                'types' => 
                array (
                  0 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'int',
                      'isIdentifier' => true,
                    ),
                  ),
                  1 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'null',
                      'isIdentifier' => true,
                    ),
                  ),
                ),
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 29,
            'endLine' => 29,
            'startColumn' => 111,
            'endColumn' => 134,
            'parameterIndex' => 3,
            'isOptional' => true,
          ),
          'model_hint' => 
          array (
            'name' => 'model_hint',
            'default' => 
            array (
              'code' => 'null',
              'attributes' => 
              array (
                'startLine' => 29,
                'endLine' => 29,
                'startTokenPos' => 76,
                'startFilePos' => 970,
                'endTokenPos' => 76,
                'endFilePos' => 973,
              ),
            ),
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionUnionType',
              'data' => 
              array (
                'types' => 
                array (
                  0 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'string',
                      'isIdentifier' => true,
                    ),
                  ),
                  1 => 
                  array (
                    'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
                    'data' => 
                    array (
                      'name' => 'null',
                      'isIdentifier' => true,
                    ),
                  ),
                ),
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 29,
            'endLine' => 29,
            'startColumn' => 137,
            'endColumn' => 162,
            'parameterIndex' => 4,
            'isOptional' => true,
          ),
        ),
        'returnsReference' => false,
        'returnType' => 
        array (
          'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
          'data' => 
          array (
            'name' => 'Liter\\Llm\\DefaultClient',
            'isIdentifier' => false,
          ),
        ),
        'attributes' => 
        array (
        ),
        'docComment' => '/**
 * Create a new LLM client with simple scalar configuration.
 *
 * This is the primary binding entry-point. All parameters except `api_key`
 * are optional — omitting them uses the same defaults as
 * [`ClientConfigBuilder`].
 *
 * # Errors
 *
 * Returns [`LiterLlmError`] if the underlying HTTP client cannot be
 * constructed, or if the resolved provider configuration is invalid.
 *
 * @param string $api_key
 * @param ?string $base_url
 * @param ?int $timeout_secs
 * @param ?int $max_retries
 * @param ?string $model_hint
 * @return DefaultClient
 * @throws \\Liter\\Llm\\LiterLlmException
 */',
        'startLine' => 29,
        'endLine' => 32,
        'startColumn' => 5,
        'endColumn' => 5,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlm',
        'implementingClassName' => 'Liter\\Llm\\LiterLlm',
        'currentClassName' => 'Liter\\Llm\\LiterLlm',
        'aliasName' => NULL,
      ),
      'createClientFromJson' => 
      array (
        'name' => 'createClientFromJson',
        'parameters' => 
        array (
          'json' => 
          array (
            'name' => 'json',
            'default' => NULL,
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
              'data' => 
              array (
                'name' => 'string',
                'isIdentifier' => true,
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 48,
            'endLine' => 48,
            'startColumn' => 49,
            'endColumn' => 60,
            'parameterIndex' => 0,
            'isOptional' => false,
          ),
        ),
        'returnsReference' => false,
        'returnType' => 
        array (
          'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
          'data' => 
          array (
            'name' => 'Liter\\Llm\\DefaultClient',
            'isIdentifier' => false,
          ),
        ),
        'attributes' => 
        array (
        ),
        'docComment' => '/**
 * Create a new LLM client from a JSON string.
 *
 * The JSON object accepts the same fields as `liter-llm.toml` (snake_case).
 *
 * # Errors
 *
 * Returns [`LiterLlmError::BadRequest`] if `json` is not valid JSON or
 * contains unknown fields.
 *
 * @param string $json
 * @return DefaultClient
 * @throws \\Liter\\Llm\\LiterLlmException
 */',
        'startLine' => 48,
        'endLine' => 51,
        'startColumn' => 5,
        'endColumn' => 5,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlm',
        'implementingClassName' => 'Liter\\Llm\\LiterLlm',
        'currentClassName' => 'Liter\\Llm\\LiterLlm',
        'aliasName' => NULL,
      ),
      'registerCustomProvider' => 
      array (
        'name' => 'registerCustomProvider',
        'parameters' => 
        array (
          'config' => 
          array (
            'name' => 'config',
            'default' => NULL,
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
              'data' => 
              array (
                'name' => 'Liter\\Llm\\CustomProviderConfig',
                'isIdentifier' => false,
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 68,
            'endLine' => 68,
            'startColumn' => 51,
            'endColumn' => 78,
            'parameterIndex' => 0,
            'isOptional' => false,
          ),
        ),
        'returnsReference' => false,
        'returnType' => 
        array (
          'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
          'data' => 
          array (
            'name' => 'void',
            'isIdentifier' => true,
          ),
        ),
        'attributes' => 
        array (
        ),
        'docComment' => '/**
 * Register a custom provider in the global runtime registry.
 *
 * The provider will be checked **before** all built-in providers during model
 * detection.  If a provider with the same `name` already exists it is replaced.
 *
 * # Errors
 *
 * Returns an error if the config is invalid (empty name, empty base_url, or
 * no model prefixes).
 *
 * @param CustomProviderConfig $config
 * @return void
 * @throws \\Liter\\Llm\\LiterLlmException
 */',
        'startLine' => 68,
        'endLine' => 71,
        'startColumn' => 5,
        'endColumn' => 5,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlm',
        'implementingClassName' => 'Liter\\Llm\\LiterLlm',
        'currentClassName' => 'Liter\\Llm\\LiterLlm',
        'aliasName' => NULL,
      ),
      'unregisterCustomProvider' => 
      array (
        'name' => 'unregisterCustomProvider',
        'parameters' => 
        array (
          'name' => 
          array (
            'name' => 'name',
            'default' => NULL,
            'type' => 
            array (
              'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
              'data' => 
              array (
                'name' => 'string',
                'isIdentifier' => true,
              ),
            ),
            'isVariadic' => false,
            'byRef' => false,
            'isPromoted' => false,
            'attributes' => 
            array (
            ),
            'startLine' => 87,
            'endLine' => 87,
            'startColumn' => 53,
            'endColumn' => 64,
            'parameterIndex' => 0,
            'isOptional' => false,
          ),
        ),
        'returnsReference' => false,
        'returnType' => 
        array (
          'class' => 'PHPStan\\BetterReflection\\Reflection\\ReflectionNamedType',
          'data' => 
          array (
            'name' => 'bool',
            'isIdentifier' => true,
          ),
        ),
        'attributes' => 
        array (
        ),
        'docComment' => '/**
 * Remove a previously registered custom provider by name.
 *
 * Returns `true` if a provider with the given name was found and removed,
 * `false` if no such provider existed.
 *
 * # Errors
 *
 * Returns an error only if the internal lock is poisoned.
 *
 * @param string $name
 * @return bool
 * @throws \\Liter\\Llm\\LiterLlmException
 */',
        'startLine' => 87,
        'endLine' => 90,
        'startColumn' => 5,
        'endColumn' => 5,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlm',
        'implementingClassName' => 'Liter\\Llm\\LiterLlm',
        'currentClassName' => 'Liter\\Llm\\LiterLlm',
        'aliasName' => NULL,
      ),
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