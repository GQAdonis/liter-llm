---
id: fixture_go_error_batch_invalid_file
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
	request := pkg.CreateBatchRequest{
		InputFileID:      ptr(`file-wrong-purpose`),
		Endpoint:         ptr(`/v1/chat/completions`),
		CompletionWindow: ptr(`24h`),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.CreateBatch(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
