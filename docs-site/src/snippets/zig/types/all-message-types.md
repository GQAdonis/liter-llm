---
id: fixture_zig_all_message_types
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/all_message_types", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.chat("{\"messages\":[{\"content\":\"You are a helpful assistant.\",\"role\":\"system\"},{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"},{\"content\":null,\"role\":\"assistant\",\"tool_calls\":[{\"function\":{\"arguments\":\"{\\\"location\\\": \\\"Paris, France\\\"}\",\"name\":\"get_weather\"},\"id\":\"call_xyz789\",\"type\":\"function\"}]},{\"content\":\"{\\\"temperature\\\": 18, \\\"unit\\\": \\\"celsius\\\", \\\"description\\\": \\\"Partly cloudy\\\"}\",\"role\":\"tool\",\"tool_call_id\":\"call_xyz789\"}],\"model\":\"gpt-4\"}");
}

```
