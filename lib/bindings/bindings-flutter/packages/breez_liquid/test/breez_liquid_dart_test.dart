import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  group('main', () {
    setUpAll(() async {
      await initApi();
    });
  });
}
