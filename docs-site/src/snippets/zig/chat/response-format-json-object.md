---
id: fixture_zig_response_format_json_object
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/response_format_json_object", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.chat("{\"messages\":[{\"content\":\"Respond with JSON only.\",\"role\":\"system\"},{\"content\":\"Give me a user object with name and age fields.\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"response_format\":{\"type\":\"json_object\"}}");
}

```
