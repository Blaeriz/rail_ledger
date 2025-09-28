import 'package:flutter/material.dart';
import 'package:mobile_scanner/mobile_scanner.dart';
import 'package:sih/insertion.dart';
import 'package:sih/publicreport.dart';

class PublicQRScannerPage extends StatefulWidget {
  @override
  State<PublicQRScannerPage> createState() => _PublicQRScannerPageState();
}

class _PublicQRScannerPageState extends State<PublicQRScannerPage> {
  bool _navigated = false;

  @override
  void initState() {
    super.initState();

    // Delay navigation after first frame
    WidgetsBinding.instance.addPostFrameCallback((_) {
      Future.delayed(const Duration(seconds: 3), () {
        if (!_navigated && mounted) {
          _navigated = true;
          Navigator.pushReplacement(
            context,
            MaterialPageRoute(builder: (_) => Publicreport()),
          );
        }
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Scan QR Code")),
      body: Stack(
        children: [
          MobileScanner(
            onDetect: (capture) {
              if (_navigated) return;

              final List<Barcode> barcodes = capture.barcodes;
              if (barcodes.isNotEmpty) {
                final String? code = barcodes.first.rawValue;
                if (code != null) {
                  print('Scanned QR code: $code');
                  // Optional: do something with code
                }
              }
            },
          ),
          const Center(
            child: CircularProgressIndicator(), // Show loader while waiting
          ),
        ],
      ),
    );
  }
}
