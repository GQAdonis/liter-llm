---
id: fixture_zig_multi_turn_conversation
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/multi_turn_conversation", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.chat("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is 2 + 2?\",\"role\":\"user\"},{\"content\":\"2 + 2 equals 4.\",\"role\":\"assistant\"},{\"content\":\"And what is 4 + 4?\",\"role\":\"user\"}],\"model\":\"gpt-4\"}");
}

```
