import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

class PublicReportsScreen extends StatefulWidget {
  static const routeName = '/public_reports';

  const PublicReportsScreen({super.key});

  @override
  State<PublicReportsScreen> createState() => _PublicReportsScreenState();
}

class _PublicReportsScreenState extends State<PublicReportsScreen> {
  List<dynamic> reports = [];
  bool loading = true;

  /// Update with your server IP
  final String baseUrl = 'http://10.12.89.179:8080';

  @override
  void initState() {
    super.initState();
    fetchReports();
  }

  Future<void> fetchReports() async {
    setState(() => loading = true);
    try {
      final response = await http.get(Uri.parse('$baseUrl/public_reports'));

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        if (mounted) setState(() => reports = data);
      } else {
        throw Exception('Failed to load reports');
      }
    } catch (e) {
      print('❌ Error fetching reports: $e');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Error fetching reports')),
        );
      }
    } finally {
      if (mounted) setState(() => loading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Public Reports'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: fetchReports,
          ),
        ],
      ),
      body: loading
          ? const Center(child: CircularProgressIndicator())
          : reports.isEmpty
              ? const Center(
                  child: Text(
                    'No reports available',
                    style: TextStyle(color: Colors.white70),
                  ),
                )
              : RefreshIndicator(
                  onRefresh: fetchReports,
                  child: ListView.builder(
                    padding: const EdgeInsets.all(8),
                    itemCount: reports.length,
                    itemBuilder: (context, index) {
                      final report = reports[index];
                      return Card(
                        color: Colors.grey[900],
                        margin: const EdgeInsets.symmetric(vertical: 6),
                        child: Padding(
                          padding: const EdgeInsets.all(12.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: report.entries.map<Widget>((entry) {
                              return Padding(
                                padding: const EdgeInsets.only(bottom: 4),
                                child: Text(
                                  '${entry.key}: ${entry.value}',
                                  style: TextStyle(
                                    color: entry.key == 'report_hash'
                                        ? Colors.white
                                        : Colors.white70,
                                    fontWeight: entry.key == 'report_hash'
                                        ? FontWeight.bold
                                        : FontWeight.normal,
                                  ),
                                ),
                              );
                            }).toList(),
                          ),
                        ),
                      );
                    },
                  ),
                ),
    );
  }
}
