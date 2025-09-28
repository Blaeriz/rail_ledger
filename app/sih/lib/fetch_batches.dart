import 'dart:convert';
import 'package:http/http.dart' as http;

class Batch {
  final int batchId;
  final int vendorId;
  final int batchSize;

  Batch({
    required this.batchId,
    required this.vendorId,
    required this.batchSize,
  });

  factory Batch.fromJson(Map<String, dynamic> json) {
    return Batch(
      batchId: json['batch_id'],
      vendorId: json['vendor_id'],
      batchSize: json['batch_size'],
    );
  }

  Map<String, dynamic> toJson() => {
        'vendor_id': vendorId,
        'batch_size': batchSize,
      };
}

class ApiService {
  static const String baseUrl = 'http://localhost:8080';

  static Future<List<Batch>> fetchBatches() async {
    final response = await http.get(Uri.parse('$baseUrl/batches'));
    if (response.statusCode == 200) {
      final List data = jsonDecode(response.body);
      return data.map((json) => Batch.fromJson(json)).toList();
    } else {
      throw Exception('Failed to fetch batches');
    }
  }

  static Future<void> addBatch(Batch batch) async {
    final response = await http.post(
      Uri.parse('$baseUrl/batches'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(batch.toJson()),
    );
    if (response.statusCode != 200) throw Exception('Failed to add batch');
  }

  static Future<void> updateBatch(int id, Batch batch) async {
    final response = await http.put(
      Uri.parse('$baseUrl/batches/$id'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode(batch.toJson()),
    );
    if (response.statusCode != 200) throw Exception('Failed to update batch');
  }

  static Future<void> deleteBatch(int id) async {
    final response = await http.delete(Uri.parse('$baseUrl/batches/$id'));
    if (response.statusCode != 200) throw Exception('Failed to delete batch');
  }
}
