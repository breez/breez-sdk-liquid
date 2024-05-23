import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';

class Balance extends StatelessWidget {
  final Stream<GetInfoResponse> walletInfoStream;

  const Balance({super.key, required this.walletInfoStream});

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<GetInfoResponse>(
      stream: walletInfoStream,
      builder: (context, walletInfoSnapshot) {
        if (walletInfoSnapshot.hasError) {
          return Center(child: Text('Error: ${walletInfoSnapshot.error}'));
        }

        if (!walletInfoSnapshot.hasData) {
          return const Center(child: Text('Loading...'));
        }

        if (walletInfoSnapshot.requireData.balanceSat.isNaN) {
          return const Center(child: Text('No balance.'));
        }
        final walletInfo = walletInfoSnapshot.data!;

        return Center(
          child: Text(
            "${walletInfo.balanceSat} sats",
            style: Theme.of(context).textTheme.headlineLarge?.copyWith(color: Colors.blue),
          ),
        );
      },
    );
  }
}
