import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:intl/intl.dart';

class PaymentItem extends StatelessWidget {
  final Payment item;
  const PaymentItem({super.key, required this.item});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      onLongPress: item.preimage != null
          ? () {
              try {
                debugPrint("Store payment preimage on clipboard. Preimage: ${item.preimage!}");
                Clipboard.setData(ClipboardData(text: item.preimage!));
                const snackBar = SnackBar(
                  behavior: SnackBarBehavior.floating,
                  content: Text('Copied payment preimage to clipboard.'),
                );
                ScaffoldMessenger.of(context).showSnackBar(snackBar);
              } catch (e) {
                final snackBar = SnackBar(
                  behavior: SnackBarBehavior.floating,
                  content: Text('Failed to copy payment preimage to clipboard. $e'),
                );
                ScaffoldMessenger.of(context).showSnackBar(snackBar);
              }
            }
          : item.swapId != null
              ? () {
                  try {
                    debugPrint("Store swap ID on clipboard. Swap ID: ${item.swapId!}");
                    Clipboard.setData(ClipboardData(text: item.swapId!));
                    const snackBar = SnackBar(
                      behavior: SnackBarBehavior.floating,
                      content: Text('Copied swap ID to clipboard.'),
                    );
                    ScaffoldMessenger.of(context).showSnackBar(snackBar);
                  } catch (e) {
                    final snackBar = SnackBar(
                      behavior: SnackBarBehavior.floating,
                      content: Text('Failed to copy payment preimage to clipboard. $e'),
                    );
                    ScaffoldMessenger.of(context).showSnackBar(snackBar);
                  }
                }
              : null,
      title: Text(_paymentTitle(item)),
      subtitle: Text(
        DateFormat('dd/MM/yyyy, HH:mm').format(
          DateTime.fromMillisecondsSinceEpoch(item.timestamp * 1000),
        ),
        style: Theme.of(context).textTheme.bodySmall,
      ),
      trailing: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Text(
            "${item.paymentType == PaymentType.send ? "-" : "+"}${item.amountSat} sats",
            style: Theme.of(context).textTheme.bodyLarge,
          ),
          if (item.feesSat != BigInt.zero) ...[
            Text("FEE: ${item.paymentType == PaymentType.receive ? "-" : ""}${item.feesSat} sats"),
          ]
        ],
      ),
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
