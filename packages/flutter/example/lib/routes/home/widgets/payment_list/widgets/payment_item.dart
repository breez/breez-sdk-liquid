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
      onLongPress: () => _onLongPress(context),
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

  void _onLongPress(BuildContext context) {
    final details = item.details;
    if (details is PaymentDetails_Lightning && details.preimage != null) {
      try {
        debugPrint("Store payment preimage on clipboard. Preimage: ${details.preimage!}");
        Clipboard.setData(ClipboardData(text: details.preimage!));
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

    if (details is PaymentDetails_Bitcoin) {
      try {
        debugPrint("Store swap ID on clipboard. Swap ID: ${details.swapId}");
        Clipboard.setData(ClipboardData(text: details.swapId));
        const snackBar = SnackBar(
          behavior: SnackBarBehavior.floating,
          content: Text('Copied swap ID to clipboard.'),
        );
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      } catch (e) {
        final snackBar = SnackBar(
          behavior: SnackBarBehavior.floating,
          content: Text('Failed to copy payment swap ID to clipboard. $e'),
        );
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      }
    }

    if (details is PaymentDetails_Liquid) {
      try {
        debugPrint("Store Liquid Address on clipboard. Liquid Address: ${details.destination}");
        Clipboard.setData(ClipboardData(text: details.destination));
        const snackBar = SnackBar(
          behavior: SnackBarBehavior.floating,
          content: Text('Copied Liquid Address to clipboard.'),
        );
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      } catch (e) {
        final snackBar = SnackBar(
          behavior: SnackBarBehavior.floating,
          content: Text('Failed to copy payment Liquid Address to clipboard. $e'),
        );
        ScaffoldMessenger.of(context).showSnackBar(snackBar);
      }
    }

    return;
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
