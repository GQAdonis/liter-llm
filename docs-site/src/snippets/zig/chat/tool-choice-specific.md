---
id: fixture_zig_tool_choice_specific
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
    const _mock_url = try std.fmt.allocPrintSentinel(std.heap.c_allocator, "{s}/fixtures/tool_choice_specific", .{if (std.c.getenv("MOCK_SERVER_URL")) |v| std.mem.span(v) else "http://localhost:8080"}, 0);
    var _client = try liter_llm.create_client("test-key", _mock_url, null, null, null);
    const _result_json = try _client.chat("{\"messages\":[{\"content\":\"What is the weather in Paris?\",\"role\":\"user\"}],\"model\":\"gpt-4\",\"tool_choice\":{\"function\":{\"name\":\"get_weather\"},\"type\":\"function\"},\"tools\":[{\"function\":{\"description\":\"Get the current weather for a given location\",\"name\":\"get_weather\",\"parameters\":{\"properties\":{\"location\":{\"description\":\"The city name\",\"type\":\"string\"}},\"required\":[\"location\"],\"type\":\"object\"}},\"type\":\"function\"},{\"function\":{\"description\":\"Search the web for information\",\"name\":\"search_web\",\"parameters\":{\"properties\":{\"query\":{\"description\":\"The search query\",\"type\":\"string\"}},\"required\":[\"query\"],\"type\":\"object\"}},\"type\":\"function\"}]}");
}

```
