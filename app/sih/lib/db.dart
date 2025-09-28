import 'package:dotenv/dotenv.dart';
import 'package:postgres_fork/postgres.dart';

class Db {
  static PostgreSQLConnection? _conn;

  static Future<PostgreSQLConnection> get connection async {
    if (_conn != null && !_conn!.isClosed) {
      return _conn!;
    }

    // Create a DotEnv instance and load the file
    final dotEnv = DotEnv()..load();

    _conn = PostgreSQLConnection(
      dotEnv['PGHOST']!,
      int.parse(dotEnv['PGPORT'] ?? '5432'),
      dotEnv['PGDATABASE']!,
      username: dotEnv['PGUSER']!,
      password: dotEnv['PGPASSWORD']!,
      useSSL: (dotEnv['PGSSL'] ?? 'true').toLowerCase() == 'true',
      timeoutInSeconds: 30,
    );

    try {
      await _conn!.open();
    } catch (e) {
      print('❌ Failed to connect to DB: $e');
      rethrow;
    }

    return _conn!;
  }

  // optional helper to force reconnect
  static Future<void> reconnect() async {
    if (_conn != null) {
      await _conn!.close();
    }
    _conn = null;
    await connection;
  }
}
