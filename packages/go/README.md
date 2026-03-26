# Liter-LM — Go

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/liter-lm">
    <img src="https://img.shields.io/crates/v/liter-lm?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://hex.pm/packages/liter_lm">
    <img src="https://img.shields.io/hexpm/v/liter_lm?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://pypi.org/project/liter-lm/">
    <img src="https://img.shields.io/pypi/v/liter-lm?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/liter-lm">
    <img src="https://img.shields.io/npm/v/liter-lm?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/liter-lm-wasm">
    <img src="https://img.shields.io/npm/v/liter-lm-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/liter-lm">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/liter-lm?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-lm/releases">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/liter-lm?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/LiterLm/">
    <img src="https://img.shields.io/nuget/v/LiterLm?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/liter-lm">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/liter-lm?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/liter-lm">
    <img src="https://img.shields.io/gem/v/liter-lm?label=Ruby&color=007ec6" alt="Ruby">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/liter-lm/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-007ec6" alt="License">
  </a>
  <a href="https://github.com/kreuzberg-dev/liter-lm">
    <img src="https://img.shields.io/badge/docs-GitHub-007ec6" alt="Documentation">
  </a>
</div>

Universal LLM API client for Go. Access 100+ LLM providers through a single interface backed by the Rust core.

> **Version 1.0.0-rc.1**
> Report issues at [github.com/kreuzberg-dev/liter-lm](https://github.com/kreuzberg-dev/liter-lm/issues).

## Install

### Using Go Modules

```bash
go get github.com/kreuzberg-dev/liter-lm/packages/go@latest
```

You'll need the native FFI library at build time. See [Building with Static Libraries](#building-with-static-libraries) below.

### Quick Start (Monorepo Development)

For development in the liter-lm monorepo:

```bash
# Build the static FFI library
cargo build -p liter-lm-ffi --release

# Go build will automatically link against the static library
cd packages/go
go build -v
```

### Building with Static Libraries

When building outside the liter-lm monorepo, provide the static library (`.a` on Unix, `.lib` on Windows).

#### Option 1: Download Pre-built Static Library

Download from [GitHub Releases](https://github.com/kreuzberg-dev/liter-lm/releases):

```bash
# Example: Linux x86_64
curl -LO https://github.com/kreuzberg-dev/liter-lm/releases/download/v1.0.0-rc.1/go-ffi-linux-x86_64.tar.gz
tar -xzf go-ffi-linux-x86_64.tar.gz

mkdir -p ~/liter-lm/lib
cp liter-lm-ffi/lib/libliter_lm_ffi.a ~/liter-lm/lib/
```

Then build with `CGO_LDFLAGS`:

```bash
# Linux/macOS
CGO_LDFLAGS="-L$HOME/liter-lm/lib -lliter_lm_ffi" go build

# Windows (MSVC)
set CGO_LDFLAGS=-L%USERPROFILE%\liter-lm\lib -lliter_lm_ffi
go build
```

#### Option 2: Build Static Library Yourself

```bash
git clone https://github.com/kreuzberg-dev/liter-lm.git
cd liter-lm

cargo build -p liter-lm-ffi --release

mkdir -p ~/liter-lm/lib
cp target/release/libliter_lm_ffi.a ~/liter-lm/lib/

cd ~/my-go-project
CGO_LDFLAGS="-L$HOME/liter-lm/lib -lliter_lm_ffi" go build
```

### System Requirements

- **Go 1.21+** required
- API keys via environment variables (e.g. `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`)

## Quickstart

```go
package main

import (
 "context"
 "fmt"
 "log"

 literlm "github.com/kreuzberg-dev/liter-lm/packages/go"
)

func main() {
 client := literlm.NewClient()

 resp, err := client.Chat(context.Background(), literlm.ChatRequest{
  Model: "openai/gpt-4o",
  Messages: []literlm.Message{
   {Role: "user", Content: "Hello!"},
  },
 })
 if err != nil {
  log.Fatalf("chat failed: %v", err)
 }

 fmt.Println(resp.Content)
}
```

Build and run:

```bash
CGO_LDFLAGS="-L$HOME/liter-lm/lib -lliter_lm_ffi" go build
./myapp
```

## Examples

### Streaming Responses

```go
stream, err := client.ChatStream(ctx, literlm.ChatRequest{
 Model:    "openai/gpt-4o",
 Messages: []literlm.Message{{Role: "user", Content: "Tell me a story"}},
})
if err != nil {
 log.Fatal(err)
}
defer stream.Close()

for chunk := range stream.Chunks() {
 fmt.Print(chunk.Delta)
}
```

### Multiple Providers

```go
// OpenAI
resp, _ := client.Chat(ctx, literlm.ChatRequest{Model: "openai/gpt-4o", Messages: msgs})

// Anthropic
resp, _ = client.Chat(ctx, literlm.ChatRequest{Model: "anthropic/claude-3-5-sonnet-20241022", Messages: msgs})

// Groq
resp, _ = client.Chat(ctx, literlm.ChatRequest{Model: "groq/llama-3.1-70b-versatile", Messages: msgs})
```

### Context-Aware Requests

```go
ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
defer cancel()

resp, err := client.Chat(ctx, literlm.ChatRequest{
 Model:    "openai/gpt-4o",
 Messages: []literlm.Message{{Role: "user", Content: "Hello!"}},
})
if err != nil {
 log.Fatalf("chat failed: %v", err)
}
fmt.Println(resp.Content)
```

## API Reference

- **GoDoc**: [pkg.go.dev/github.com/kreuzberg-dev/liter-lm/packages/go](https://pkg.go.dev/github.com/kreuzberg-dev/liter-lm/packages/go)
- **Provider Registry**: [schemas/providers.json](https://github.com/kreuzberg-dev/liter-lm/blob/main/schemas/providers.json)
- **GitHub Repository**: [github.com/kreuzberg-dev/liter-lm](https://github.com/kreuzberg-dev/liter-lm)

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `ld returned 1 exit status` or `undefined reference to 'liter_lm_...'` | Static library not found. Set `CGO_LDFLAGS="-L/path/to/lib -lliter_lm_ffi" go build` |
| `cannot find -lliter_lm_ffi` | Download from [GitHub Releases](https://github.com/kreuzberg-dev/liter-lm/releases) or build: `cargo build -p liter-lm-ffi --release` |
| `401 Unauthorized` | API key not set. Export `OPENAI_API_KEY` (or equivalent) before running. |
| `unknown provider` | Check the [provider registry](https://github.com/kreuzberg-dev/liter-lm/blob/main/schemas/providers.json) for the correct prefix. |

## Testing / Tooling

- `task go:lint` — runs `gofmt` and `golangci-lint`
- `task go:test` — executes `go test ./...` (after building the static FFI library)
- `task e2e:go:verify` — regenerates fixtures and runs `go test ./...` inside `e2e/go`

Need help? Open an issue at [github.com/kreuzberg-dev/liter-lm/issues](https://github.com/kreuzberg-dev/liter-lm/issues).
