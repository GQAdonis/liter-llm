import 'package:liter_llm/liter_llm.dart';
import 'package:test/test.dart';

void main() {
  test('AuthConfig equality is based on authType and envVar', () {
    const a = AuthConfig(authType: AuthType.bearer, envVar: 'OPENAI_API_KEY');
    const b = AuthConfig(authType: AuthType.bearer, envVar: 'OPENAI_API_KEY');
    const c = AuthConfig(authType: AuthType.apiKey, envVar: 'OPENAI_API_KEY');

    expect(a, equals(b));
    expect(a.hashCode, equals(b.hashCode));
    expect(a, isNot(equals(c)));
  });
}
