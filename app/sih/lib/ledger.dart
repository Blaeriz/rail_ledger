import 'package:flutter/material.dart';
import 'csv_storage.dart';

class LedgerScreen extends StatefulWidget {
  const LedgerScreen({super.key});

  @override
  State<LedgerScreen> createState() => _LedgerScreenState();
}

class _LedgerScreenState extends State<LedgerScreen> {
  List<Map<String, dynamic>> reports = [];

  @override
  void initState() {
    super.initState();
    loadReports();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    loadReports(); // refresh when reopened
  }

  Future<void> loadReports() async {
    final data = await CSVStorage.readReports();
    setState(() {
      reports = data;
    });
  }

  void showReportDetails(Map<String, dynamic> report) {
    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: Text("📋 Batch ${report['batch_id']}"),
        content: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: report.entries.map((entry) {
              return Padding(
                padding: const EdgeInsets.symmetric(vertical: 4),
                child: Text(
                  "${entry.key}: ${entry.value}",
                  style: const TextStyle(fontSize: 14),
                ),
              );
            }).toList(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text("Close"),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Ledger Reports'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: loadReports,
          ),
        ],
      ),
      body: reports.isEmpty
          ? const Center(child: Text('No reports found'))
          : ListView.builder(
              padding: const EdgeInsets.all(12),
              itemCount: reports.length,
              itemBuilder: (context, index) {
                final report = reports[index];
                return GestureDetector(
                  onTap: () => showReportDetails(report),
                  child: Card(
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                    elevation: 4,
                    margin: const EdgeInsets.symmetric(vertical: 6),
                    child: ListTile(
                      leading: const Icon(Icons.qr_code, color: Colors.blue),
                      title: Text(
                        "Batch ID: ${report['batch_id'] ?? 'N/A'}",
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          fontSize: 16,
                        ),
                      ),
                      subtitle: Text(
                        "Vendor: ${report['vendor_id'] ?? 'Unknown'}\n"
                        "QC: ${report['qc_status'] ?? 'N/A'}",
                      ),
                      trailing: const Icon(Icons.arrow_forward_ios, size: 16),
                    ),
                  ),
                );
              },
            ),
    );
  }
}
