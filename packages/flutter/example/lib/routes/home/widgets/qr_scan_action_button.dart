import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/qr_scan/barcode_scanner_simple.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/send_payment/send_payment_dialog.dart';

class QrActionButton extends StatelessWidget {
  final BindingLiquidSdk liquidSDK;

  const QrActionButton({super.key, required this.liquidSDK});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(top: 32.0, bottom: 0),
      child: FloatingActionButton(
        backgroundColor: Colors.white,
        shape: const StadiumBorder(),
        onPressed: () => _scanBarcode(context),
        child: const Icon(
          Icons.qr_code_2,
          size: 32,
          color: Colors.blue,
        ),
      ),
    );
  }

  void _scanBarcode(BuildContext context) {
    debugPrint("Scanning for QR Code");
    Navigator.of(context).push<String?>(
      MaterialPageRoute(builder: (context) {
        return const BarcodeScanner();
      }),
    ).then((barcode) {
      if (barcode == null || barcode.isEmpty) return;
      debugPrint("Scanned string: '$barcode'");
      showDialog(
        context: context,
        builder: (context) => SendPaymentDialog(barcodeValue: barcode, liquidSdk: liquidSDK),
      );
    });
  }
}
