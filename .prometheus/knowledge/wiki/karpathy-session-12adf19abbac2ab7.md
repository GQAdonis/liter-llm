---
type: SessionRecord
id: karpathy-session-12adf19abbac2ab7
title: Karpathy session 12adf19abbac
tags:
- karpathy
- session-learning
sources:
- resource: session:971902b0-35f4-4989-9164-e4fa8a916515
generated:
  by: pk/1.11.0
  at: 2026-10-19T11:52:24.522843+00:00
created_at: 2026-10-19T11:52:24.522843+00:00
updated_at: 2026-10-19T11:52:24.522843+00:00
revision: 0
---

## Delta

I sent one PR upstream, plus four issues instead of more PRs.

## PR #291: stream chat responses as they arrive

Upstream had a performance bug in the Tower `LlmService` layer, the middleware stack used for budgets, cost tracking, rate limits and fallback. It collected the **entire** chat stream into memory before returning anything. A client using that stack got no tokens until the model had finished generating, so "streaming" only behaved like streaming without the middleware.

The fix had two parts:

1. **Pass the stream straight through.** `LlmClient::chat_stream` already returns a stream that can outlive the request, so the buffering wasn't needed.
2. **Fix a crash risk this exposed.** Spend tracking settles cost when the stream is dropped. If a client disconnected early, the old code checked once for a final usage chunk using a do-nothing waker:
   - On a live network stream, that check finds nothing ready, so spend went unrecorded.
   - It could also panic inside Tokio and abort the process.

   The fix spawns a small background task that drains the rest of the stream and records the real usage. If no Tokio runtime is available, it logs a warning and records what it has.

It also added tests on real sockets (the first chunk arrives before the upstream finishes, and dropping the stream closes the connection), plus doc and changelog updates.

The fork had already made the pass-through change. The drain fix was new and also went into the fork as PR #6.

## Why the rest were issues, not PRs

The other fork changes either touched upstream code that had since moved on, or were built around the fork's own needs. Issues let the maintainer choose the design:

| Issue | Problem |
|---|---|
| #292 | `refusal: null` was re-sent in chat history, and strict providers such as Groq reject it |
| #293 | No way to make the HTTP client ignore machine-level proxy settings |
| #294 | No CLI command to validate a proxy config without starting the server |
| #295 | A stream cut off mid-response ended as if it had finished normally |

The maintainer implemented all four within a day and shipped them in 2.3.0. That's why the 2.3.0 merge mostly kept upstream's versions over the fork's.

## Root Cause

No explicit root-cause section was captured; preserve this as a session record, not an inferred diagnosis.

## Corrective Actions

Review and promote only reusable findings.

## Session Metadata

- Harness: claude-code
- Session: 971902b0-35f4-4989-9164-e4fa8a916515
- Captured: 2026-10-19T11:51:46.266022Z
- Project: /Volumes/T7/Projects/references/liter-llm

## Changed Paths

- No changed paths detected.
