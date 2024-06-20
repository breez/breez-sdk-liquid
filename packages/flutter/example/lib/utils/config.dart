import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:path_provider/path_provider.dart';

Future<Config> getConfig({
  LiquidNetwork network = LiquidNetwork.mainnet,
}) async {
  debugPrint("Getting default SDK config for network: $network");
  final defaultConf = defaultConfig(network: network);
  debugPrint("Getting SDK config");
  final workingDir = await getApplicationDocumentsDirectory();
  return defaultConf.copyWith(
    workingDir: workingDir.path,
  );
}

extension ConfigCopyWith on Config {
  Config copyWith({
    String? boltzUrl,
    String? electrumUrl,
    String? workingDir,
    LiquidNetwork? network,
    BigInt? paymentTimeoutSec,
  }) {
    return Config(
      boltzUrl: boltzUrl ?? this.boltzUrl,
      electrumUrl: electrumUrl ?? this.electrumUrl,
      workingDir: workingDir ?? this.workingDir,
      network: network ?? this.network,
      paymentTimeoutSec: paymentTimeoutSec ?? this.paymentTimeoutSec,
    );
  }
}
