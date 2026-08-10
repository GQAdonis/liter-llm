---
id: fixture_go_smoke_moderate_batch
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
		Input: ptr(pkg.ModerationInput(`["Hello world","Nice weather today"]`)),
		Model: ptr(`omni-moderation-latest`),
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
