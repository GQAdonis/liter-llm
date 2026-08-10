---
id: fixture_go_error_moderate_bad_request
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	"fmt"
	pkg "github.com/xberg-io/liter-llm/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	request := pkg.ModerationRequest{
		Input: ptr(pkg.ModerationInput(`Hello`)),
		Model: ptr(`nonexistent-moderation`),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Moderate(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
