import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initialize();
  await initializeWallet();
  runApp(const MyApp());
}

const String mnemonic = "";

Future initializeWallet() async {
  assert(mnemonic.isNotEmpty, "Please enter your mnemonic.");
  final dataDir = await getApplicationDocumentsDirectory();
  final req = ConnectRequest(
    mnemonic: mnemonic,
    dataDir: dataDir.path,
    network: Network.liquid,
  );
  await connect(req: req);
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Breez Liquid Native Packages'),
        ),
        body: Padding(
          padding: const EdgeInsets.all(10),
          child: SingleChildScrollView(
            child: Column(
              children: [
                FutureBuilder<GetInfoResponse>(
                  future: getInfo(
                    req: const GetInfoRequest(
                      withScan: true,
                    ),
                  ),
                  initialData: null,
                  builder: (context, snapshot) {
                    if (snapshot.hasError) {
                      return Text('Error: ${snapshot.error}');
                    }

                    if (!snapshot.hasData) {
                      return const Text('Loading...');
                    }

                    if (snapshot.requireData.balanceSat.isNaN) {
                      return const Text('No balance.');
                    }
                    final walletInfo = snapshot.data!;

                    return Column(
                      children: [
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 16.0),
                          child: Text(
                            "Balance",
                            style: Theme.of(context).textTheme.headlineSmall,
                          ),
                        ),
                        Padding(
                          padding: const EdgeInsets.symmetric(vertical: 32.0),
                          child: Center(
                            child: Text(
                              "${walletInfo.balanceSat} sats",
                              style: Theme.of(context).textTheme.headlineSmall,
                            ),
                          ),
                        ),
                        ListTile(
                          title: Text(
                            "pubKey: ${walletInfo.pubkey}",
                            style: Theme.of(context).textTheme.bodySmall,
                          ),
                        ),
                      ],
                    );
                  },
                ),
                const SizedBox(height: 16.0),
                FutureBuilder<PrepareReceiveResponse>(
                  future: prepareReceivePayment(
                    req: const PrepareReceiveRequest(payerAmountSat: 1000),
                  ),
                  initialData: null,
                  builder: (context, snapshot) {
                    if (snapshot.hasError) {
                      return Text('Error: ${snapshot.error}');
                    }

                    if (!snapshot.hasData) {
                      return const Text('Loading...');
                    }

                    if (snapshot.requireData.pairHash.isEmpty) {
                      return const Text('No pair hash.');
                    }
                    final prepareReceiveResponse = snapshot.data!;
                    debugPrint(prepareReceiveResponse.pairHash);

                    return Column(
                      children: [
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 16.0),
                          child: Text(
                            "Preparing a receive payment of 1000 sats",
                            style: Theme.of(context).textTheme.headlineSmall,
                          ),
                        ),
                        ListTile(
                          title: Text(
                              "Pair Hash: ${prepareReceiveResponse.pairHash}"),
                        ),
                        ListTile(
                          title: Text(
                              "Payer Amount: ${prepareReceiveResponse.payerAmountSat} (in sats)"),
                        ),
                        ListTile(
                          title: Text(
                              "Fees: ${prepareReceiveResponse.feesSat} (in sats)"),
                        ),
                        const SizedBox(height: 16.0),
                        FutureBuilder<ReceivePaymentResponse>(
                          future: receivePayment(req: prepareReceiveResponse),
                          initialData: null,
                          builder: (context, snapshot) {
                            if (snapshot.hasError) {
                              return Text('Error: ${snapshot.error}');
                            }

                            if (!snapshot.hasData) {
                              return const Text('Loading...');
                            }

                            if (snapshot.requireData.id.isEmpty) {
                              return const Text('Missing invoice id');
                            }

                            final receivePaymentResponse = snapshot.data!;
                            debugPrint(
                                "Invoice ID: ${receivePaymentResponse.id}");
                            debugPrint(
                                "Invoice: ${receivePaymentResponse.invoice}");

                            return Column(
                              children: [
                                Padding(
                                  padding: const EdgeInsets.symmetric(
                                      horizontal: 16.0),
                                  child: Text(
                                    "Invoice for receive payment of 1000 sats",
                                    style: Theme.of(context)
                                        .textTheme
                                        .headlineSmall,
                                  ),
                                ),
                                ListTile(
                                  title: Text(
                                      "Invoice ID: ${receivePaymentResponse.id}"),
                                ),
                                ListTile(
                                  title: Text(
                                      "Invoice: ${receivePaymentResponse.invoice}"),
                                ),
                              ],
                            );
                          },
                        ),
                      ],
                    );
                  },
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
