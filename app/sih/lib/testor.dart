import 'package:flutter/material.dart';
import 'package:sih/connectivity/neon_home.dart';

class Testor extends StatefulWidget {
  const Testor({super.key});

  @override
  State<Testor> createState() => _TestorState();
}

class _TestorState extends State<Testor> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: ElevatedButton(
          onPressed: () async {
            await Neonconnection().connect();
          },
          child: Text("Test Neon Connection"),
        ),
      ),
    );
  }
}
