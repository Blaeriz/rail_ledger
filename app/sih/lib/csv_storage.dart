import 'dart:io';
import 'package:path_provider/path_provider.dart';
import 'package:csv/csv.dart';

class CSVStorage {
  static const String fileName = 'ledger.csv';
  static const List<String> headers = [
    'batch_id','vendor_id','batch_size','data_of_production','qc_status',
    'expiry_date','last_inspection_date','fitment_date','fitment_location','qr_hash'
  ];

  // Get the CSV file, create if it doesn't exist
  static Future<File> _getFile() async {
    final dir = await getApplicationDocumentsDirectory();
    final file = File('${dir.path}/$fileName');

    if (!await file.exists()) {
      final csvString = const ListToCsvConverter().convert([headers]);
      await file.writeAsString(csvString);
    }

    return file;
  }

  // Append a new report
  static Future<void> appendReport(Map<String, dynamic> report) async {
    final file = await _getFile();
    final csvString = await file.readAsString();

    List<List<dynamic>> rows = const CsvToListConverter().convert(csvString);
    rows.add(headers.map((h) => report[h] ?? '').toList());

    final newCsv = const ListToCsvConverter().convert(rows);
    await file.writeAsString(newCsv);
  }

  static Future<List<Map<String, dynamic>>> readReports() async {
  final file = await _getFile();
  final csvString = await file.readAsString();
  if (csvString.isEmpty) return [];

  final rows = const CsvToListConverter().convert(csvString);
  final csvHeaders = rows.first.map((e) => e.toString()).toList();
  final dataRows = rows.sublist(1);

  return dataRows.map((row) {
    final map = <String, dynamic>{};
    for (int i = 0; i < csvHeaders.length; i++) {
      map[csvHeaders[i]] = (i < row.length ? row[i] : '');
    }
    return map;
  }).toList();
}

}
