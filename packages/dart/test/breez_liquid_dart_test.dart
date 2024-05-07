import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  group('main', () {
    setUpAll(() async {
      await initApi();
    });

    test("after setting up, getInfo should throw exception with 'Not initialized' message", () async {
      GetInfoRequest req = GetInfoRequest(withScan: true);
      try {
        await getInfo(req: req);
      } catch (e) {
        if (e is AnyhowException) {
          expect(e.message, "Not initialized");
        }
      }
    });
  });
}
