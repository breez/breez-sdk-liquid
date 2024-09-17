import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:qr_flutter/qr_flutter.dart';

class ReceivePaymentDialog extends StatefulWidget {
  final BindingLiquidSdk liquidSDK;
  final Stream<List<Payment>> paymentsStream;

  const ReceivePaymentDialog({super.key, required this.liquidSDK, required this.paymentsStream});

  @override
  State<ReceivePaymentDialog> createState() => _ReceivePaymentDialogState();
}

class _ReceivePaymentDialogState extends State<ReceivePaymentDialog> {
  final TextEditingController payerAmountController = TextEditingController();

  int? payerAmountSat;
  int? feesSat;
  bool creatingInvoice = false;

  String? invoiceDestination;

  StreamSubscription<List<Payment>>? streamSubscription;

  @override
  void initState() {
    super.initState();
    streamSubscription = widget.paymentsStream.listen((paymentList) {
      if (invoiceDestination != null && invoiceDestination!.isNotEmpty) {
        if (paymentList.any(
          (e) {
            final details = e.details;
            if (details is PaymentDetails_Lightning && details.preimage != null) {
              return details.preimage! == invoiceDestination!;
            }
            return false;
          },
        )) {
          debugPrint("Payment Received! Id: $invoiceDestination");
          if (mounted) {
            Navigator.of(context).pop();
          }
        }
      }
    });
  }

  @override
  void dispose() {
    streamSubscription?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: creatingInvoice ? null : Text(invoiceDestination != null ? "Invoice" : "Receive Payment"),
      content: creatingInvoice || invoiceDestination != null
          ? Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                if (invoiceDestination != null) ...[
                  AspectRatio(
                    aspectRatio: 1,
                    child: SizedBox(
                      width: 200.0,
                      height: 200.0,
                      child: QrImageView(
                        embeddedImage: const AssetImage("assets/icons/app_icon.png"),
                        data: invoiceDestination!.toUpperCase(),
                        size: 200.0,
                      ),
                    ),
                  ),
                  if (payerAmountSat != null && feesSat != null) ...[
                    Padding(
                      padding: const EdgeInsets.only(left: 8.0, right: 8.0),
                      child: Row(
                        mainAxisSize: MainAxisSize.max,
                        children: [
                          const Text('Payer Amount:'),
                          const Expanded(child: SizedBox(width: 0)),
                          Text('$payerAmountSat sats'),
                        ],
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.only(left: 8.0, right: 8.0),
                      child: Row(
                        mainAxisSize: MainAxisSize.max,
                        children: [
                          const Text('Payer Fees:'),
                          const Expanded(child: SizedBox(width: 0)),
                          Text('$feesSat sats'),
                        ],
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.only(left: 8.0, right: 8.0),
                      child: Row(
                        mainAxisSize: MainAxisSize.max,
                        children: [
                          const Text('Receive Amount:'),
                          const Expanded(child: SizedBox(width: 0)),
                          Text('${payerAmountSat! - feesSat!} sats'),
                        ],
                      ),
                    ),
                  ]
                ],
                if (creatingInvoice) ...[
                  const Text('Creating Invoice...'),
                  const SizedBox(height: 16),
                  const CircularProgressIndicator(color: Colors.blue),
                ]
              ],
            )
          : TextField(
              controller: payerAmountController,
              decoration: const InputDecoration(label: Text("Enter payer amount in sats")),
              inputFormatters: [FilteringTextInputFormatter.digitsOnly],
              keyboardType: TextInputType.number,
            ),
      actions: creatingInvoice
          ? []
          : [
              TextButton(
                child: const Text("CANCEL"),
                onPressed: () {
                  Navigator.of(context).pop();
                },
              ),
              if (invoiceDestination == null) ...[
                TextButton(
                  onPressed: () async {
                    try {
                      setState(() => creatingInvoice = true);
                      int amountSat = int.parse(payerAmountController.text);
                      PrepareReceiveRequest prepareReceiveReq = PrepareReceiveRequest(
                        paymentMethod: PaymentMethod.lightning,
                        payerAmountSat: BigInt.from(amountSat),
                      );
                      PrepareReceiveResponse prepareResponse = await widget.liquidSDK.prepareReceivePayment(
                        req: prepareReceiveReq,
                      );
                      setState(() {
                        payerAmountSat = prepareResponse.payerAmountSat?.toInt();
                        feesSat = prepareResponse.feesSat.toInt();
                      });
                      ReceivePaymentRequest receiveReq = ReceivePaymentRequest(
                        prepareResponse: prepareResponse,
                      );
                      ReceivePaymentResponse resp = await widget.liquidSDK.receivePayment(req: receiveReq);
                      debugPrint(
                        "Created Invoice for $payerAmountSat sats with $feesSat sats fees.\nDestination:${resp.destination}",
                      );
                      setState(() => invoiceDestination = resp.destination);
                    } catch (e) {
                      setState(() {
                        payerAmountSat = null;
                        feesSat = null;
                        invoiceDestination = null;
                      });
                      final errMsg = "Error receiving payment: $e";
                      debugPrint(errMsg);
                      if (context.mounted) {
                        final snackBar = SnackBar(
                          behavior: SnackBarBehavior.floating,
                          content: Text(errMsg),
                        );
                        ScaffoldMessenger.of(context).showSnackBar(snackBar);
                      }
                    } finally {
                      setState(() => creatingInvoice = false);
                    }
                  },
                  child: const Text("OK"),
                ),
              ]
            ],
    );
  }
}
