import 'package:flutter/material.dart';
import 'csv_storage.dart'; // import the CSV helper

class InspectionForm extends StatefulWidget {
  @override
  _InspectionFormState createState() => _InspectionFormState();
}

class _InspectionFormState extends State<InspectionForm> {
  final _formKey = GlobalKey<FormState>();

  final Map<String, dynamic> _formData = {
  'batch_id': '',
  'vendor_id': '',
  'batch_size': 0,
  'date_of_production': '', // ✅ fixed spelling
  'qc_status': '',
  'expiry_date': '',
  'last_inspection_date': '',
  'fitment_date': '',
  'fitment_location': '',
  'qr_hash': '',
};


  /// Show date picker and save result
  Future<void> _pickDate(BuildContext context, String key) async {
    final picked = await showDatePicker(
      context: context,
      initialDate: DateTime.now(),
      firstDate: DateTime(2000),
      lastDate: DateTime(2100),
    );
    if (picked != null) {
      setState(() {
        _formData[key] = picked.toIso8601String().split('T').first;
      });
    }
  }

  Future<void> submitReport() async {
    try {
      await CSVStorage.appendReport(_formData);
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('✅ Report saved locally to CSV!')),
      );

      // Clear form
      _formData.updateAll((key, value) => key == 'batch_size' ? 0 : '');
      setState(() {});
    } catch (e) {
      print('error $e');
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed to save report: $e')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Inspection Form')),
      body: Form(
        key: _formKey,
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: [
            // Section: General Info
            const Text(
              "Batch Information",
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),

            _buildTextField("batch_id", Icons.qr_code),
            _buildTextField("vendor_id", Icons.store),
            _buildNumberField("batch_size", Icons.confirmation_number),

            const SizedBox(height: 20),

            // Section: Dates
            const Text(
              "Dates",
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),

           _buildDateField("date_of_production", Icons.date_range),

            _buildDateField("expiry_date", Icons.calendar_today),
            _buildDateField("last_inspection_date", Icons.history),
            _buildDateField("fitment_date", Icons.build),

            const SizedBox(height: 20),

            // Section: Quality & Fitment
            const Text(
              "Quality & Fitment",
              style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),

            _buildTextField("qc_status", Icons.verified),
            _buildTextField("fitment_location", Icons.location_on),
            _buildTextField("qr_hash", Icons.fingerprint),

            const SizedBox(height: 30),

            ElevatedButton.icon(
              onPressed: submitReport,
              icon: const Icon(Icons.save),
              label: const Text("Submit Report"),
              style: ElevatedButton.styleFrom(
                padding: const EdgeInsets.symmetric(vertical: 14),
                textStyle: const TextStyle(fontSize: 18),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12),
                ),
              ),
            ),
            SizedBox(height: 30),
          ],
        ),
      ),
    );
  }

  Widget _buildTextField(String key, IconData icon) {
    return Card(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: const BorderSide(color: Colors.blueAccent, width: 1),
      ),
      margin: const EdgeInsets.symmetric(vertical: 6),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
        child: TextFormField(
          decoration: InputDecoration(
            icon: Icon(icon, color: Colors.blueAccent),
            labelText: key.replaceAll('_', ' ').toUpperCase(),
            border: InputBorder.none,
          ),
          onChanged: (v) => _formData[key] = v,
          initialValue: _formData[key].toString(),
        ),
      ),
    );
  }

  Widget _buildNumberField(String key, IconData icon) {
    return Card(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: const BorderSide(color: Colors.green, width: 1),
      ),
      margin: const EdgeInsets.symmetric(vertical: 6),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
        child: TextFormField(
          keyboardType: TextInputType.number,
          decoration: InputDecoration(
            icon: Icon(icon, color: Colors.green),
            labelText: key.replaceAll('_', ' ').toUpperCase(),
            border: InputBorder.none,
          ),
          onChanged: (v) => _formData[key] = int.tryParse(v) ?? 0,
          initialValue: _formData[key].toString(),
        ),
      ),
    );
  }

  /// Reusable Date Picker Field
  Widget _buildDateField(String key, IconData icon) {
    return Card(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: const BorderSide(color: Colors.orange, width: 1),
      ),
      margin: const EdgeInsets.symmetric(vertical: 6),
      child: ListTile(
        leading: Icon(icon, color: Colors.orange),
        title: Text(
          _formData[key].isEmpty
              ? "Select ${key.replaceAll('_', ' ').toUpperCase()}"
              : _formData[key],
        ),
        trailing: const Icon(Icons.arrow_drop_down),
        onTap: () => _pickDate(context, key),
      ),
    );
  }
}
