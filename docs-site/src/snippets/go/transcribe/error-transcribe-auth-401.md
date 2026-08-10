---
id: fixture_go_error_transcribe_auth_401
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
	request := pkg.CreateTranscriptionRequest{
		Model: ptr(`whisper-1`),
		File:  ptr(`audio.mp3`),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Transcribe(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
