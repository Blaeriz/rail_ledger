import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

class Publicreport extends StatefulWidget {
  static const routeName = '/insert_publicreport';

  const Publicreport({super.key});

  @override
  State<Publicreport> createState() => _PublicreportState();
}

class _PublicreportState extends State<Publicreport> {
  final _formKey = GlobalKey<FormState>();

  final Map<String, dynamic> formData = {
    'report_hash': '',
    'reported_by': '',
    'photo_url': '',
    'qr_hash': '',
    'remarks': '',
    'aadhaar': '',
  };

  bool submitting = false;

  Future<void> submitReport() async {
    if (!_formKey.currentState!.validate()) return;
    _formKey.currentState!.save();

    setState(() => submitting = true);
    try {
      final response = await http.post(
        Uri.parse('http://10.12.89.179:8080/insert_publicreport'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode(formData),
      );

      final data = jsonDecode(response.body);
      if (response.statusCode == 200) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(data['message'] ?? 'Report inserted')),
        );
        _formKey.currentState!.reset();
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
        title: const Text('Insert Public Report'),
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
                _buildTextField('Report Hash', 'report_hash'),
                _buildTextField('Reported By', 'reported_by'),
                _buildTextField('Photo URL', 'photo_url'),
                _buildTextField('QR Hash', 'qr_hash'),
                _buildTextField('Remarks', 'remarks'),
                _buildTextField(
                  'Aadhaar Number',
                  'aadhaar',
                  keyboardType: TextInputType.number,
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
                            style: TextStyle(
                                fontSize: 18, fontWeight: FontWeight.bold),
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
          contentPadding:
              const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: BorderSide.none,
          ),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: const BorderSide(color: Colors.orange, width: 2),
          ),
        ),
        validator: (value) =>
            value == null || value.isEmpty ? 'Required' : null,
        onSaved: (value) => formData[keyName] = value,
      ),
    );
  }
}
