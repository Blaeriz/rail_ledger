// import 'package:flutter/material.dart';
// import 'package:sih/fetch_batches.dart';


// class BatchesScreen extends StatefulWidget {
//   @override
//   _BatchesScreenState createState() => _BatchesScreenState();
// }

// class _BatchesScreenState extends State<BatchesScreen> {
//   late Future<List<Batch>> _batches;

//   @override
//   void initState() {
//     super.initState();
//     _batches = ApiService.fetchBatches();
//   }

//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       appBar: AppBar(title: Text('Batches')),
//       body: FutureBuilder<List<Batch>>(
//         future: _batches,
//         builder: (context, snapshot) {
//           if (snapshot.connectionState == ConnectionState.waiting) {
//             return Center(child: CircularProgressIndicator());
//           } else if (snapshot.hasError) {
//             return Center(child: Text('Error: ${snapshot.error}'));
//           } else if (!snapshot.hasData || snapshot.data!.isEmpty) {
//             return Center(child: Text('No batches found'));
//           }

//           final batches = snapshot.data!;
//           return ListView.builder(
//             itemCount: batches.length,
//             itemBuilder: (context, index) {
//               final batch = batches[index];
//               return ListTile(
//                 title: Text('Batch ID: ${batch.batchId}'),
//                 subtitle: Text(
//                     'Vendor ID: ${batch.vendorId}, Size: ${batch.batchSize}'),
//               );
//             },
//           );
//         },
//       ),
//     );
//   }
// }
