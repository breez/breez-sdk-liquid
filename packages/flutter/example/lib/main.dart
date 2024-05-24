import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/connect/connect_page.dart';
import 'package:flutter_breez_liquid_example/routes/home/home_page.dart';
import 'package:flutter_breez_liquid_example/services/credentials_manager.dart';
import 'package:flutter_breez_liquid_example/services/keychain.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initialize();
  final credentialsManager = CredentialsManager(keyChain: KeyChain());
  final mnemonic = await credentialsManager.restoreMnemonic();
  BindingLiquidSdk? liquidSDK;
  if (mnemonic.isNotEmpty) {
    liquidSDK = await reconnect(mnemonic: mnemonic);
  }
  runApp(App(credentialsManager: credentialsManager, liquidSDK: liquidSDK));
}

Future<BindingLiquidSdk> reconnect({
  required String mnemonic,
  Network network = Network.liquid,
}) async {
  final dataDir = await getApplicationDocumentsDirectory();
  final req = ConnectRequest(
    mnemonic: mnemonic,
    dataDir: dataDir.path,
    network: network,
  );
  return await connect(req: req);
}

class App extends StatefulWidget {
  final CredentialsManager credentialsManager;
  final BindingLiquidSdk? liquidSDK;
  const App({super.key, required this.credentialsManager, this.liquidSDK});

  static const title = 'Breez Liquid SDK Demo';

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: App.title,
      theme: ThemeData.from(colorScheme: ColorScheme.fromSeed(seedColor: Colors.white), useMaterial3: true),
      home: widget.liquidSDK == null
          ? ConnectPage(credentialsManager: widget.credentialsManager)
          : HomePage(credentialsManager: widget.credentialsManager, liquidSDK: widget.liquidSDK!),
    );
  }
}
