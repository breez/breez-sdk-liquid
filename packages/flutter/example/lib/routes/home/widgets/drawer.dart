import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/mnemonics_dialog.dart';
import 'package:flutter_breez_liquid_example/services/credentials_manager.dart';

class HomePageDrawer extends StatefulWidget {
  final CredentialsManager credentialsManager;
  final BindingLiquidSdk liquidSDK;

  const HomePageDrawer({super.key, required this.liquidSDK, required this.credentialsManager});

  @override
  State<HomePageDrawer> createState() => _HomePageDrawerState();
}

class _HomePageDrawerState extends State<HomePageDrawer> {
  @override
  Widget build(BuildContext context) {
    return Drawer(
      backgroundColor: Colors.blue,
      child: ListView(
        padding: EdgeInsets.zero,
        children: [
          const DrawerHeader(
            curve: Curves.fastOutSlowIn,
            child: Text(
              'Breez Liquid SDK Demo',
              style: TextStyle(fontSize: 16.0, color: Colors.white),
            ),
          ),
          ListTile(
            enabled: false,
            leading: const Icon(Icons.backup_outlined),
            title: const Text('Backup'),
            titleTextStyle:
                const TextStyle(fontSize: 16.0, color: Colors.white, decoration: TextDecoration.lineThrough),
            onTap: () async {
              try {
                debugPrint("Creating backup.");
                // TODO: Backup API should return backup file or it's filepath
                widget.liquidSDK.backup(req: const BackupRequest());
                debugPrint("Created backup.");
              } catch (e) {
                final errMsg = "Failed to create backup. $e";
                debugPrint(errMsg);
                if (context.mounted) {
                  final snackBar = SnackBar(behavior: SnackBarBehavior.floating, content: Text(errMsg));
                  ScaffoldMessenger.of(context).showSnackBar(snackBar);
                }
              }
            },
          ),
          ListTile(
            enabled: false,
            leading: const Icon(Icons.restore),
            title: const Text('Restore'),
            titleTextStyle:
                const TextStyle(fontSize: 16.0, color: Colors.white, decoration: TextDecoration.lineThrough),
            onTap: () async {
              try {
                debugPrint("Restoring backup.");
                // TODO: Select backup file to restore
                RestoreRequest req = const RestoreRequest();
                widget.liquidSDK.restore(req: req);
                debugPrint("Restored backup.");
              } catch (e) {
                final errMsg = "Failed to restore backup. $e";
                debugPrint(errMsg);
                if (context.mounted) {
                  final snackBar = SnackBar(behavior: SnackBarBehavior.floating, content: Text(errMsg));
                  ScaffoldMessenger.of(context).showSnackBar(snackBar);
                }
              }
            },
          ),
          ListTile(
            leading: const Icon(Icons.cached, color: Colors.white),
            title: const Text('Empty Wallet Cache'),
            titleTextStyle: const TextStyle(fontSize: 16.0, color: Colors.white),
            onTap: () async {
              try {
                debugPrint("Emptying wallet cache.");
                widget.liquidSDK.emptyWalletCache();
                debugPrint("Emptied wallet cache.");
              } catch (e) {
                final errMsg = "Failed to empty wallet cache. $e";
                debugPrint(errMsg);
                if (context.mounted) {
                  final snackBar = SnackBar(behavior: SnackBarBehavior.floating, content: Text(errMsg));
                  ScaffoldMessenger.of(context).showSnackBar(snackBar);
                }
              }
            },
          ),
          ListTile(
            leading: const Icon(Icons.info_outline, color: Colors.white),
            title: const Text('Display Mnemonics'),
            titleTextStyle: const TextStyle(fontSize: 16.0, color: Colors.white),
            onTap: () async {
              try {
                await widget.credentialsManager.restoreMnemonic().then((mnemonics) {
                  showDialog(
                    context: context,
                    builder: (context) => MnemonicsDialog(
                      mnemonics: mnemonics.split(" "),
                    ),
                  );
                });
              } on Exception catch (e) {
                final errMsg = "Failed to display mnemonics. $e";
                debugPrint(errMsg);
                if (context.mounted) {
                  final snackBar = SnackBar(behavior: SnackBarBehavior.floating, content: Text(errMsg));
                  ScaffoldMessenger.of(context).showSnackBar(snackBar);
                }
              }
            },
          )
        ],
      ),
    );
  }
}
