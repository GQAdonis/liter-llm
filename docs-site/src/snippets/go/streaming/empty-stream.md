---
id: fixture_go_empty_stream
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
	request := pkg.ChatCompletionRequest{
		Model:  ptr(`gpt-4`),
		Stream: true,
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.ChatStream(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
