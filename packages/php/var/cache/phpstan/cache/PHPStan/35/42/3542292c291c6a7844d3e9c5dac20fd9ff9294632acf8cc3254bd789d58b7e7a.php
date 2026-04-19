<?php declare(strict_types = 1);

// osfsl-/Users/naamanhirschfeld/workspace/kreuzberg-dev/liter-llm/packages/php/stubs/liter_llm_extension.php-PHPStan\BetterReflection\Reflection\ReflectionClass-Liter\Llm\LiterLlmApi
return \PHPStan\Cache\CacheItem::__set_state(array(
   'variableKey' => 'v2-2bdafbd6b337dee2441af7db0d29f0a46acf71417691c212879329d5062c0635-8.4.20-6.70.0.0',
   'data' => 
  array (
    'locatedSource' => 
    array (
      'class' => 'PHPStan\\BetterReflection\\SourceLocator\\Located\\LocatedSource',
      'data' => 
      array (
        'name' => 'Liter\\Llm\\LiterLlmApi',
        'filename' => '/Users/naamanhirschfeld/workspace/kreuzberg-dev/liter-llm/packages/php/stubs/liter_llm_extension.php',
      ),
    ),
    'namespace' => 'Liter\\Llm',
    'name' => 'Liter\\Llm\\LiterLlmApi',
    'shortName' => 'LiterLlmApi',
    'isInterface' => false,
    'isTrait' => false,
    'isEnum' => false,
    'isBackedEnum' => false,
    'modifiers' => 0,
    'docComment' => NULL,
    'attributes' => 
    array (
    ),
    'startLine' => 1439,
    'endLine' => 1445,
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
            'startLine' => 1441,
            'endLine' => 1441,
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
                'startLine' => 1441,
                'endLine' => 1441,
                'startTokenPos' => 8557,
                'startFilePos' => 34322,
                'endTokenPos' => 8557,
                'endFilePos' => 34325,
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
            'startLine' => 1441,
            'endLine' => 1441,
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
                'startLine' => 1441,
                'endLine' => 1441,
                'startTokenPos' => 8567,
                'startFilePos' => 34349,
                'endTokenPos' => 8567,
                'endFilePos' => 34352,
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
            'startLine' => 1441,
            'endLine' => 1441,
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
                'startLine' => 1441,
                'endLine' => 1441,
                'startTokenPos' => 8577,
                'startFilePos' => 34375,
                'endTokenPos' => 8577,
                'endFilePos' => 34378,
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
            'startLine' => 1441,
            'endLine' => 1441,
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
                'startLine' => 1441,
                'endLine' => 1441,
                'startTokenPos' => 8587,
                'startFilePos' => 34403,
                'endTokenPos' => 8587,
                'endFilePos' => 34406,
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
            'startLine' => 1441,
            'endLine' => 1441,
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
        'docComment' => NULL,
        'startLine' => 1441,
        'endLine' => 1441,
        'startColumn' => 5,
        'endColumn' => 193,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlmApi',
        'implementingClassName' => 'Liter\\Llm\\LiterLlmApi',
        'currentClassName' => 'Liter\\Llm\\LiterLlmApi',
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
            'startLine' => 1442,
            'endLine' => 1442,
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
        'docComment' => NULL,
        'startLine' => 1442,
        'endLine' => 1442,
        'startColumn' => 5,
        'endColumn' => 91,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlmApi',
        'implementingClassName' => 'Liter\\Llm\\LiterLlmApi',
        'currentClassName' => 'Liter\\Llm\\LiterLlmApi',
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
            'startLine' => 1443,
            'endLine' => 1443,
            'startColumn' => 51,
            'endColumn' => 89,
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
        'docComment' => NULL,
        'startLine' => 1443,
        'endLine' => 1443,
        'startColumn' => 5,
        'endColumn' => 100,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlmApi',
        'implementingClassName' => 'Liter\\Llm\\LiterLlmApi',
        'currentClassName' => 'Liter\\Llm\\LiterLlmApi',
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
            'startLine' => 1444,
            'endLine' => 1444,
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
        'docComment' => NULL,
        'startLine' => 1444,
        'endLine' => 1444,
        'startColumn' => 5,
        'endColumn' => 75,
        'couldThrow' => false,
        'isClosure' => false,
        'isGenerator' => false,
        'isVariadic' => false,
        'modifiers' => 17,
        'namespace' => 'Liter\\Llm',
        'declaringClassName' => 'Liter\\Llm\\LiterLlmApi',
        'implementingClassName' => 'Liter\\Llm\\LiterLlmApi',
        'currentClassName' => 'Liter\\Llm\\LiterLlmApi',
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