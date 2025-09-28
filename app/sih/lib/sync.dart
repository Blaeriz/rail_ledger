// import 'package:connectivity_plus/connectivity_plus.dart';
// import 'package:http/http.dart' as http;
// import 'dart:convert';

// import 'package:sih/db.dart';

// Future<bool> hasInternet() async {
//   final conn = await Connectivity().checkConnectivity();
//   return conn != ConnectivityResult.none;
// }

// Future<void> syncPendingReports() async {
//   final pending = await LocalDb.instance.getPendingReports();
//   for (var report in pending) {
//     try {
//       final response = await http.post(
//         Uri.parse('https://your-server.com/api/reports'),
//         headers: {'Content-Type': 'application/json'},
//         body: jsonEncode(report),
//       );
//       if (response.statusCode == 200 || response.statusCode == 201) {
//         await LocalDb.instance.markAsSynced(report['id']);
//       }
//     } catch (e) {
//       // Still offline, try again later
//       print('Sync failed: $e');
//     }
//   }
// }
