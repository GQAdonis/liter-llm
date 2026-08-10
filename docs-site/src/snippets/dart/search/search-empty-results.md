---
id: fixture_dart_search_empty_results
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:liter_llm/liter_llm.dart';
Future<void> main() async {
  final _request = SearchRequest(model: 'brave/web-search', query: 'xyznonexistent12345xyz', maxResults: null, searchDomainFilter: null, country: null);
  final _mockUrl = _fixtureUrl("search_empty_results");
      final _client = await LiterLlmBridge.createClient('test-key', baseUrl: _mockUrl);
  final result = await _client.search(req: _request);
}

```
