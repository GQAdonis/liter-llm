---
id: fixture_go_edge_image_multiple_n
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
	request := pkg.CreateImageRequest{
		Prompt: ptr(`A cat`),
		Model:  ptr(`dall-e-3`),
		N:      3,
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.ImageGenerate(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
