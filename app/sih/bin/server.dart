import 'dart:io';
import 'dart:convert';

import 'package:dotenv/dotenv.dart';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_router/shelf_router.dart';
import 'package:shelf_cors_headers/shelf_cors_headers.dart';

import '../lib/db.dart';

void main(List<String> args) async {
  final dotEnv = DotEnv()..load();
  final port = int.parse(dotEnv['PORT'] ?? '8080');
  final schema = dotEnv['PGSCHEMA'] ?? 'public';
  final batchTable = dotEnv['PGTABLE'] ?? 'batch_info';
  final reportTable = 'inspection_reports';
  final publicreportTable = 'public_reports';

  final router = Router()
    ..get('/', _health)
    ..get('/batches', (req) => _getAllBatches(req, schema, batchTable))
    ..get('/public_reports', (req) => _getAllReports(req, schema, publicreportTable))
    ..get('/batch_info', (req) => _getBatchInfo(req, schema, batchTable))
    ..post('/insert_report', (req) => _insertReport(req, schema, reportTable))
    ..post('/insert_publicreport', (req) => _insertReport(req, schema, publicreportTable));

  final handler = const Pipeline()
      .addMiddleware(corsHeaders())
      .addMiddleware(logRequests())
      .addHandler(router);

  final server = await serve(handler, InternetAddress.anyIPv4, port);
  print('✅ Neon API running on http://${server.address.host}:${server.port}');
}

// JSON encoder helper for DateTime
dynamic _toEncodable(dynamic item) {
  if (item is DateTime) return item.toIso8601String();
  return item;
}

// --- Routes ---
Future<Response> _health(Request req) async {
  try {
    final conn = await Db.connection;
    await conn.execute('SELECT 1;');
    return Response.ok('OK');
  } catch (e, st) {
    print('❌ Health check failed: $e');
    print(st);
    return Response.internalServerError(body: e.toString());
  }
}

Future<Response> _getAllReports(Request req, String schema, String table) async {
  try {
    final conn = await Db.connection;
    final result = await conn.mappedResultsQuery('SELECT * FROM $schema.$table;');
    final rows = result.map((m) => m.values.first).toList();

    return Response.ok(
      jsonEncode(rows, toEncodable: _toEncodable),
      headers: {'Content-Type': 'application/json'},
    );
  } catch (e) {
    print('❌ Fetch failed: $e');
    return Response.internalServerError(
      body: jsonEncode({'error': e.toString()}),
      headers: {'Content-Type': 'application/json'},
    );
  }
}

Future<Response> _getAllBatches(Request req, String schema, String table) async {
  try {
    final conn = await Db.connection;
    final result = await conn.mappedResultsQuery('SELECT * FROM $schema.$table LIMIT 50;');
    final rows = result.map((m) => m.values.first).toList();

    return Response.ok(
      jsonEncode(rows, toEncodable: _toEncodable),
      headers: {'Content-Type': 'application/json'},
    );
  } catch (e) {
    return Response.internalServerError(body: e.toString());
  }
}

Future<Response> _getBatchInfo(Request req, String schema, String table) async {
  try {
    final conn = await Db.connection;
    final result = await conn.mappedResultsQuery('SELECT * FROM $schema.$table;');
    final rows = result.map((m) => m.values.first).toList();

    return Response.ok(
      jsonEncode(rows, toEncodable: _toEncodable),
      headers: {'Content-Type': 'application/json'},
    );
  } catch (e) {
    return Response.internalServerError(body: e.toString());
  }
}

Future<Response> _insertReport(Request req, String schema, String table) async {
  try {
    final body = await req.readAsString();
    final data = jsonDecode(body);

    // Define required fields for each table
    final requiredFields = table == 'inspection_reports'
        ? ['batch_id', 'inspector_name', 'remark', 'status', 'email']
        : ['report_hash', 'reported_by', 'qr_hash', 'aadhaar'];

    // Check for missing fields
    for (var field in requiredFields) {
      if (data[field] == null || (data[field] is String && data[field].isEmpty)) {
        return Response(
          400,
          body: jsonEncode({'error': 'Missing required field: $field'}),
          headers: {'Content-Type': 'application/json'},
        );
      }
    }

    final conn = await Db.connection;

    await conn.transaction((ctx) async {
      if (table == 'inspection_reports') {
        await ctx.query(
          '''
          INSERT INTO $schema.$table (batch_id, inspector_name, remark, status, email)
          VALUES (@batchId, @inspectorName, @remark, @status, @email)
          ''',
          substitutionValues: {
            'batchId': data['batch_id'],
            'inspectorName': data['inspector_name'],
            'remark': data['remark'],
            'status': data['status'],
            'email': data['email'],
          },
        );
      } else if (table == 'public_reports') {
        await ctx.query(
          '''
          INSERT INTO $schema.$table (report_hash, reported_by, photo_url, qr_hash, remarks, aadhaar)
          VALUES (@reportHash, @reportedBy, @photoUrl, @qrHash, @remarks, @aadhaar)
          ''',
          substitutionValues: {
            'reportHash': data['report_hash'],
            'reportedBy': data['reported_by'],
            'photoUrl': data['photo_url'],
            'qrHash': data['qr_hash'],
            'remarks': data['remarks'],
            'aadhaar': int.tryParse(data['aadhaar'].toString()) ?? 0,
          },
        );
      }
    });

    print('✅ Report inserted into $table');

    return Response.ok(
      jsonEncode({'message': '✅ Report inserted successfully into $table'}),
      headers: {'Content-Type': 'application/json'},
    );
  } catch (e, st) {
    print('❌ Insert failed: $e');
    print(st);
    return Response.internalServerError(
      body: jsonEncode({'error': e.toString()}),
      headers: {'Content-Type': 'application/json'},
    );
  }
}
