---
id: fixture_go_smoke_image_basic
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
		Prompt: ptr(`A white cat sitting on a windowsill`),
		Model:  ptr(`dall-e-3`),
		N:      1,
		Size:   ptr(`1024x1024`),
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
