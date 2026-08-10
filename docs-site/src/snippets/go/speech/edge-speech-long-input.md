---
id: fixture_go_edge_speech_long_input
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
		Model: ptr(`tts-1`),
		Input: ptr(`This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. This is a long input text. End of input.`),
		Voice: ptr(`echo`),
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
