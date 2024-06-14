import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/receive_payment/receive_payment_dialog.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/send_payment/send_payment_dialog.dart';

class HomePageBottomAppBar extends StatelessWidget {
  final BindingLiquidSdk liquidSDK;
  final Stream<List<Payment>> paymentsStream;
  const HomePageBottomAppBar({super.key, required this.liquidSDK, required this.paymentsStream});

  @override
  Widget build(BuildContext context) {
    return BottomAppBar(
      color: Colors.blue,
      height: 60,
      child: Row(
        mainAxisSize: MainAxisSize.max,
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: [
          Expanded(
            child: TextButton(
              style: TextButton.styleFrom(
                padding: EdgeInsets.zero,
                foregroundColor: Colors.white,
              ),
              onPressed: () {
                showDialog(
                  context: context,
                  builder: (context) => SendPaymentDialog(liquidSdk: liquidSDK),
                );
              },
              child: Text(
                "SEND",
                textAlign: TextAlign.center,
                style: Theme.of(context).primaryTextTheme.titleMedium,
                maxLines: 1,
              ),
            ),
          ),
          Container(width: 64),
          Expanded(
            child: TextButton(
              style: TextButton.styleFrom(
                padding: EdgeInsets.zero,
                foregroundColor: Colors.white,
              ),
              onPressed: () {
                showDialog(
                  context: context,
                  builder: (context) => ReceivePaymentDialog(
                    liquidSDK: liquidSDK,
                    paymentsStream: paymentsStream,
                  ),
                );
              },
              child: Text(
                "RECEIVE",
                textAlign: TextAlign.center,
                maxLines: 1,
                style: Theme.of(context).primaryTextTheme.titleMedium,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
