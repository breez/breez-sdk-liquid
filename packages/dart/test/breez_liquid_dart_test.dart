import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  late BindingLiquidSdk sdk;

  group('main', () {
    setUpAll(() async {
      await initApi();
      ConnectRequest connectRequest = ConnectRequest(mnemonic: "", network: Network.liquidTestnet);
      sdk = await connect(req: connectRequest);
    });

    test("after setting up, getInfo should throw exception with 'Not initialized' message", () async {
      try {
        await sdk.getInfo();
      } catch (e) {
        if (e is AnyhowException) {
          expect(e.message, "Not initialized");
        }
      }
    });
  });
}
