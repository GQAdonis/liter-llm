---
id: fixture_zig_context_window_exceeded
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/context_window_exceeded", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = _client.chat("{\"messages\":[{\"content\":\"Very long prompt that exceeds the context window...\",\"role\":\"user\"}],\"model\":\"gpt-4\"}") catch |err| {
        std.debug.print("call failed as expected: {s}\n", .{@errorName(err)});
        return;
    };
    _ = _result_json;
}

```
