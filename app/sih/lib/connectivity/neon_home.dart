import 'package:postgres/postgres.dart';
import 'package:postgres_fork/postgres.dart';


class Neonconnection{
  Future<void> connect() async {
  final conn = PostgreSQLConnection(
    'ep-spring-poetry-a18e7nwj-pooler.ap-southeast-1.aws.neon.tech',
    5432,
    'thank_god',
  );

  await conn.open();
  print("connected to neon");

  List<List<dynamic>> res = await conn.query(
    'SELECT batch_id, vendor_id, batch_size FROM batch_info;',
  );

  for (var row in res) {
    print("Row:$row");
  }

  await conn.close();
}

}
