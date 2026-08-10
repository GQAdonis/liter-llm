---
id: fixture_zig_smoke_response_with_tools
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/smoke_response_with_tools", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.create_response("{\"input\":\"What is the weather in San Francisco?\",\"model\":\"gpt-4o\",\"tools\":[{\"description\":\"Get current weather for a location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"},\"type\":\"function\"}]}");
}

```
