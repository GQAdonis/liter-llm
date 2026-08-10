---
id: fixture_go_error_speech_bad_model
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
	request := pkg.CreateSpeechRequest{
		Model: ptr(`tts-nonexistent`),
		Input: ptr(`Hello`),
		Voice: ptr(`alloy`),
	}
		client, clientErr := pkg.CreateClient("your-api-key", nil, nil, nil, nil)
	if clientErr != nil {
		panic(clientErr)
	}
	result, err := client.Speech(request)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
