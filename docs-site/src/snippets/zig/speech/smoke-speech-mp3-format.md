---
id: fixture_zig_smoke_speech_mp3_format
language: zig
target: zig
level: typecheck
requires: []
side_effect: safe
---

```zig title="Zig"
const std = @import("std");
const liter_llm = @import("liter_llm");

pub fn main() !void {
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/smoke_speech_mp3_format", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    _ = try _client.speech("{\"input\":\"The quick brown fox jumps over the lazy dog.\",\"model\":\"tts-1-hd\",\"response_format\":\"mp3\",\"speed\":1.0,\"voice\":\"nova\"}");
}

```
