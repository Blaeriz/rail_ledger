import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

class InsertReportScreen extends StatefulWidget {
  static const routeName = '/insert_report';

  const InsertReportScreen({super.key});

  @override
  State<InsertReportScreen> createState() => _InsertReportScreenState();
}

class _InsertReportScreenState extends State<InsertReportScreen> {
  final _formKey = GlobalKey<FormState>();

  final Map<String, dynamic> formData = {
    'batch_id': '',
    'inspector_name': '',
    'remark': '',
    'status': null,
    'email': '',
  };

  bool submitting = false;

  Future<void> submitReport() async {
    if (!_formKey.currentState!.validate()) return;
    _formKey.currentState!.save();

    setState(() => submitting = true);
    try {
      final response = await http.post(
        Uri.parse('http://10.12.89.179:8080/insert_report'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(formData),
      );

      final data = jsonDecode(response.body);
      if (response.statusCode == 200) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(data['message'] ?? 'Report inserted')),
        );
        _formKey.currentState!.reset();
        setState(() => formData['status'] = null); // reset dropdown
      } else {
        throw Exception(data['error'] ?? 'Failed to insert report');
      }
    } catch (e) {
      print('❌ Insert failed: $e');
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error inserting report: $e')),
      );
    } finally {
      setState(() => submitting = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.grey[900],
      appBar: AppBar(
        title: const Text('Insert Report'),
        backgroundColor: Colors.grey[850],
        elevation: 2,
      ),
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
        child: Form(
          key: _formKey,
          child: SingleChildScrollView(
            child: Column(
              children: [
                _buildTextField('Batch ID', 'batch_id'),
                _buildTextField('Inspector Name', 'inspector_name'),
                _buildTextField('Remark', 'remark'),
                _buildStatusDropdown(),
                _buildTextField(
                  'Email',
                  'email',
                  keyboardType: TextInputType.emailAddress,
                ),
                const SizedBox(height: 30),
                SizedBox(
                  width: double.infinity,
                  height: 50,
                  child: ElevatedButton(
                    onPressed: submitting ? null : submitReport,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.orange[600],
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                      elevation: 4,
                    ),
                    child: submitting
                        ? const CircularProgressIndicator(color: Colors.white)
                        : const Text(
                            'Submit',
                            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                          ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildTextField(
    String label,
    String keyName, {
    TextInputType keyboardType = TextInputType.text,
  }) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: TextFormField(
        keyboardType: keyboardType,
        style: const TextStyle(color: Colors.white),
        decoration: InputDecoration(
          labelText: label,
          labelStyle: const TextStyle(color: Colors.white70),
          filled: true,
          fillColor: Colors.grey[850],
          contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: BorderSide.none,
          ),
        ),
        validator: (value) => value == null || value.isEmpty ? 'Required' : null,
        onSaved: (value) => formData[keyName] = value,
      ),
    );
  }

  Widget _buildStatusDropdown() {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: DropdownButtonFormField<int>(
        value: formData['status'],
        decoration: InputDecoration(
          labelText: 'Status',
          labelStyle: const TextStyle(color: Colors.white70),
          filled: true,
          fillColor: Colors.grey[850],
          contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: BorderSide.none,
          ),
        ),
        dropdownColor: Colors.grey[900],
        items: const [
          DropdownMenuItem(value: 1, child: Text("Pass")),
          DropdownMenuItem(value: 0, child: Text("Fail")),
        ],
        onChanged: (val) {
          setState(() => formData['status'] = val);
        },
        validator: (value) => value == null ? 'Required' : null,
        onSaved: (value) => formData['status'] = value,
      ),
    );
  }
}
