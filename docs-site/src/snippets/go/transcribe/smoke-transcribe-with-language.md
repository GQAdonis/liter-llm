---
id: fixture_go_smoke_transcribe_with_language
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
		Model:    ptr(`whisper-1`),
		File:     ptr(`audio_de.mp3`),
		Language: ptr(`de`),
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
