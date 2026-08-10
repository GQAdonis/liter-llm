---
id: fixture_go_error_file_bad_purpose
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
	request := pkg.CreateFileRequest{
		File:    ptr(`data.jsonl`),
		Purpose: ptr(pkg.FilePurpose(`invalid-purpose`)),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.CreateFile(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
