import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/balance.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/bottom_app_bar.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/mnemonics_dialog.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/payment_list/payment_list.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/qr_scan_action_button.dart';
import 'package:flutter_breez_liquid_example/services/credentials_manager.dart';

class HomePage extends StatefulWidget {
  final CredentialsManager credentialsManager;
  final BindingLiquidSdk liquidSDK;

  const HomePage({super.key, required this.credentialsManager, required this.liquidSDK});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  Stream<GetInfoResponse> walletInfoStream() async* {
    debugPrint("Initialized walletInfoStream");
    GetInfoRequest req = const GetInfoRequest(withScan: true);
    yield await widget.liquidSDK.getInfo(req: req);
    while (true) {
      await Future.delayed(const Duration(seconds: 10));
      GetInfoRequest req = const GetInfoRequest(withScan: false);
      yield await widget.liquidSDK.getInfo(req: req);
      debugPrint("Refreshed wallet info");
    }
  }

  Stream<List<Payment>> paymentsStream() async* {
    debugPrint("Initialized paymentsStream");
    yield await widget.liquidSDK.listPayments();
    while (true) {
      await Future.delayed(const Duration(seconds: 10));
      yield await widget.liquidSDK.listPayments();
      debugPrint("Refreshed payments");
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Breez Liquid SDK Demo'),
          backgroundColor: Colors.white,
          foregroundColor: Colors.blue,
          actions: [
            IconButton(
              onPressed: () async {
                await widget.credentialsManager.restoreMnemonic().then((mnemonics) {
                  showDialog(
                    context: context,
                    builder: (context) => MnemonicsDialog(
                      mnemonics: mnemonics.split(" "),
                    ),
                  );
                });
              },
              icon: const Icon(Icons.info_outline),
            )
          ],
        ),
        body: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
            return Column(
              mainAxisSize: MainAxisSize.min,
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                Container(
                  height: constraints.maxHeight * 0.3,
                  color: Colors.white,
                  child: Balance(walletInfoStream: walletInfoStream()),
                ),
                Container(
                  height: constraints.maxHeight * 0.7,
                  color: Colors.white,
                  child: PaymentList(
                    paymentsStream: paymentsStream(),
                    onRefresh: () async => await _sync(),
                  ),
                ),
              ],
            );
          },
        ),
        floatingActionButton: QrActionButton(liquidSDK: widget.liquidSDK),
        floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
        bottomNavigationBar: HomePageBottomAppBar(
          liquidSDK: widget.liquidSDK,
          paymentsStream: paymentsStream(),
        ),
      ),
    );
  }

  Future<void> _sync() async {
    try {
      debugPrint("Syncing wallet.");
      await widget.liquidSDK.sync();
      debugPrint("Wallet synced!");
    } on Exception catch (e) {
      debugPrint("Failed to sync wallet. $e");
    }
  }
}
