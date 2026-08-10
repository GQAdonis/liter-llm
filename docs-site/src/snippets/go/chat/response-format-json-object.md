---
id: fixture_go_response_format_json_object
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
		Model:          ptr(`gpt-4`),
		ResponseFormat: ptr(pkg.ResponseFormat(`{"type":"json_object"}`)),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Chat(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
