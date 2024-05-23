import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';

class PaymentItem extends StatelessWidget {
  final Payment item;
  const PaymentItem({super.key, required this.item});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      title: Text(_paymentTitle(item)),
      trailing: Text("${item.amountSat} sats"),
    );
  }

  String _paymentTitle(Payment payment) {
    final paymentType = payment.paymentType;

    switch (paymentType) {
      case PaymentType.receive:
        return "Received Payment";
      case PaymentType.send:
        return "Sent Payment";
    }
  }
}
