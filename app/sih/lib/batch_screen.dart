import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:sih/insertion.dart';

class BatchesScreen extends StatefulWidget {
  static const routeName = '/batches';

  const BatchesScreen({super.key});

  @override
  State<BatchesScreen> createState() => _BatchesScreenState();
}

class _BatchesScreenState extends State<BatchesScreen> {
  List<dynamic> batches = [];
  bool loading = true;

  /// Base API URL
  final String baseUrl = 'http://10.12.89.179:8080';

  @override
  void initState() {
    super.initState();
    fetchBatches();
  }

  Future<void> fetchBatches() async {
    setState(() => loading = true);
    try {
      final response = await http.get(Uri.parse('$baseUrl/batches'));

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        if (mounted) setState(() => batches = data);
      } else {
        throw Exception('Failed to load batches');
      }
    } catch (e) {
      print('❌ Error fetching batches: $e');
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Error fetching batches')),
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
        title: const Text('Batches'),
        actions: [
          IconButton(
            icon: const Icon(Icons.add),
            onPressed: () async {
              await Navigator.push(context, MaterialPageRoute(builder: (_)=>InsertReportScreen()));
              fetchBatches();
            },
          ),
        ],
      ),
      body: loading
          ? const Center(child: CircularProgressIndicator())
          : RefreshIndicator(
              onRefresh: fetchBatches,
              child: batches.isEmpty
                  ? const Center(
                      child: Text(
                        'No batches available',
                        style: TextStyle(color: Colors.white70),
                      ),
                    )
                  : ListView.builder(
                      padding: const EdgeInsets.all(8),
                      itemCount: batches.length,
                      itemBuilder: (context, index) {
                        final batch = batches[index];
                        return Card(
                          color: Colors.grey[900],
                          margin: const EdgeInsets.symmetric(vertical: 6),
                          child: Padding(
                            padding: const EdgeInsets.all(12.0),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: batch.entries.map<Widget>((entry) {
                                return Padding(
                                  padding: const EdgeInsets.only(bottom: 4),
                                  child: Text(
                                    '${entry.key}: ${entry.value}',
                                    style: TextStyle(
                                      color: entry.key == 'batch_id'
                                          ? Colors.white
                                          : Colors.white70,
                                      fontWeight: entry.key == 'batch_id'
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
