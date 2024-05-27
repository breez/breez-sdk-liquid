import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/qr_scan/scan_overlay.dart';
import 'package:flutter_breez_liquid_example/routes/home/widgets/qr_scan/scanner_error_widget.dart';
import 'package:mobile_scanner/mobile_scanner.dart';

class BarcodeScanner extends StatefulWidget {
  const BarcodeScanner({super.key});

  @override
  State<BarcodeScanner> createState() => _BarcodeScannerState();
}

class _BarcodeScannerState extends State<BarcodeScanner> with WidgetsBindingObserver {
  bool popped = false;

  final MobileScannerController controller = MobileScannerController(
    autoStart: false,
    torchEnabled: false,
    useNewCameraSelector: true,
  );

  Barcode? _barcode;
  StreamSubscription<Object?>? _subscription;

  void _handleBarcode(BarcodeCapture barcodes) {
    if (mounted) {
      setState(() {
        _barcode = barcodes.barcodes.firstOrNull;
      });
      if (popped) {
        debugPrint("Skipping, already popped");
        return;
      }
      popped = true;
      final code = _barcode?.rawValue;
      if (code == null) {
        debugPrint("Failed to scan QR code.");
      } else {
        popped = true;
        debugPrint("Popping read QR code: $code");
        Navigator.of(context).pop(code);
      }
    }
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);

    _subscription = controller.barcodes.listen(_handleBarcode);

    unawaited(controller.start());
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (!controller.value.isInitialized) {
      return;
    }

    switch (state) {
      case AppLifecycleState.detached:
      case AppLifecycleState.hidden:
      case AppLifecycleState.paused:
        return;
      case AppLifecycleState.resumed:
        _subscription = controller.barcodes.listen(_handleBarcode);

        unawaited(controller.start());
      case AppLifecycleState.inactive:
        unawaited(_subscription?.cancel());
        _subscription = null;
        unawaited(controller.stop());
    }
  }

  @override
  Widget build(BuildContext context) {
    var scanWindowDimension = MediaQuery.of(context).size.width - 72;
    return Scaffold(
      body: Stack(
        children: [
          MobileScanner(
            scanWindow: Rect.fromCenter(
              center: MediaQuery.sizeOf(context).center(Offset.zero),
              width: scanWindowDimension,
              height: scanWindowDimension,
            ),
            controller: controller,
            errorBuilder: (context, error, child) {
              return ScannerErrorWidget(error: error);
            },
            overlayBuilder: (context, constraints) {
              return const ScanOverlay();
            },
            fit: BoxFit.cover,
          ),
        ],
      ),
    );
  }

  @override
  Future<void> dispose() async {
    WidgetsBinding.instance.removeObserver(this);
    unawaited(_subscription?.cancel());
    _subscription = null;
    super.dispose();
    await controller.dispose();
  }
}
