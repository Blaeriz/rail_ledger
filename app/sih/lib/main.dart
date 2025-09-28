// lib/main.dart
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:sih/connectivity/neon_home.dart';
import 'package:sih/insertion.dart';
import 'package:sih/inspector.dart';
import 'package:sih/ledger.dart';
import 'package:sih/public_reports_screen.dart';
import 'package:sih/publicreport.dart';
import 'package:sih/qr_public.dart';
import 'package:sih/qrcode.dart';
import 'package:sih/testor.dart';
import 'package:sih/batch_screen.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Login',
      theme: ThemeData.dark().copyWith(
        useMaterial3: true,
        colorScheme: ColorScheme.dark(
          primary: Colors.tealAccent.shade200,
          secondary: Colors.tealAccent,
          surface: const Color(0xFF121212),
          background: const Color(0xFF0F0F12),
        ),
        inputDecorationTheme: const InputDecorationTheme(
          filled: true,
          fillColor: Color(0xFF1A1A1A),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(12)),
          ),
        ),
        elevatedButtonTheme: ElevatedButtonThemeData(
          style: ElevatedButton.styleFrom(
            backgroundColor: Colors.tealAccent.shade200,
            foregroundColor: Colors.black,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(10),
            ),
            padding: const EdgeInsets.symmetric(vertical: 14, horizontal: 20),
          ),
        ),
      ),
      home: const LoginShell(),
      debugShowCheckedModeBanner: false,
    );
  }
}

enum UserRole { public, inspector }

class LoginShell extends StatefulWidget {
  const LoginShell({super.key});
  @override
  State<LoginShell> createState() => _LoginShellState();
}

class _LoginShellState extends State<LoginShell> {
  UserRole _role = UserRole.public;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.background,
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.symmetric(horizontal: 22, vertical: 36),
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 520),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const SizedBox(height: 6),
                  const Icon(
                    Icons.train_rounded,
                    size: 64,
                    color: Colors.tealAccent,
                  ),
                  const SizedBox(height: 12),
                  Text(
                    'Rail Ledger',
                    style: Theme.of(context).textTheme.headlineSmall!.copyWith(
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                  const SizedBox(height: 6),
                  Text(
                    'Sign in to continue',
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                  const SizedBox(height: 28),
                  _buildRoleToggle(),
                  const SizedBox(height: 22),
                  Card(
                    color: Theme.of(context).colorScheme.surface,
                    elevation: 6,
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(14),
                    ),
                    child: Padding(
                      padding: const EdgeInsets.all(18),
                      child: _role == UserRole.public
                          ? AadhaarLogin()
                          : InspectorLogin(),
                    ),
                  ),
                  const SizedBox(height: 14),
                  Text(
                    _role == UserRole.public
                        ? 'General public: scan or use Aadhaar to quickly report emergencies.'
                        : 'Inspectors: sign in with your official email and password.',
                    style: Theme.of(context).textTheme.bodySmall!.copyWith(
                      color: Colors.grey.shade400,
                    ),
                    textAlign: TextAlign.center,
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildRoleToggle() {
    return Container(
      padding: const EdgeInsets.all(4),
      decoration: BoxDecoration(
        color: const Color(0xFF141414),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          _roleButton(UserRole.public, 'Aadhaar (Public)', Icons.person),
          _roleButton(UserRole.inspector, 'Inspector', Icons.badge),
        ],
      ),
    );
  }

  Widget _roleButton(UserRole role, String label, IconData icon) {
    final bool active = _role == role;
    return Material(
      color: active ? Colors.tealAccent.shade200 : Colors.transparent,
      borderRadius: BorderRadius.circular(10),
      child: InkWell(
        borderRadius: BorderRadius.circular(10),
        onTap: () => setState(() => _role = role),
        child: Container(
          padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 14),
          child: Row(
            children: [
              Icon(
                icon,
                size: 18,
                color: active ? Colors.black : Colors.white70,
              ),
              const SizedBox(width: 8),
              Text(
                label,
                style: TextStyle(
                  fontSize: 14,
                  color: active ? Colors.black : Colors.white70,
                  fontWeight: active ? FontWeight.w700 : FontWeight.w500,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// *************** Aadhaar flow (public) ***************
class AadhaarLogin extends StatefulWidget {
  @override
  State<AadhaarLogin> createState() => _AadhaarLoginState();
}

class _AadhaarLoginState extends State<AadhaarLogin> {
  final _formKey = GlobalKey<FormState>();
  final TextEditingController _aadhaarController = TextEditingController();
  final TextEditingController _otpController = TextEditingController();

  bool _loading = false;
  bool _otpSent = false;

  @override
  void dispose() {
    _aadhaarController.dispose();
    _otpController.dispose();
    super.dispose();
  }

  Future<void> _sendOtp() async {
    if (!_formKey.currentState!.validate()) return;

    setState(() => _loading = true);
    await Future.delayed(const Duration(milliseconds: 900));
    setState(() {
      _loading = false;
      _otpSent = true;
    });
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('OTP sent to registered mobile (demo)')),
    );
  }

  Future<void> _verifyOtpAndLogin() async {
    if (_otpController.text.trim().length < 4) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(const SnackBar(content: Text('Please enter a valid OTP')));
      return;
    }

    setState(() => _loading = true);
    await Future.delayed(const Duration(milliseconds: 900));
    setState(() => _loading = false);

    // Navigate to Public Home
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(
        builder: (_) => PublicHomePage(aadhaar: _aadhaarController.text.trim()),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(
        children: [
          const SizedBox(height: 6),
          Text(
            'Sign in with Aadhaar',
            style: Theme.of(
              context,
            ).textTheme.titleMedium!.copyWith(fontWeight: FontWeight.w700),
          ),
          const SizedBox(height: 12),
          TextFormField(
            controller: _aadhaarController,
            keyboardType: TextInputType.number,
            inputFormatters: [
              FilteringTextInputFormatter.digitsOnly,
              LengthLimitingTextInputFormatter(12),
            ],
            decoration: const InputDecoration(
              labelText: 'Aadhaar number',
              hintText: 'Enter 12-digit Aadhaar',
              prefixIcon: Icon(Icons.credit_card_rounded),
            ),
            validator: (v) {
              if (v == null || v.trim().length != 12)
                return 'Enter a valid 12-digit Aadhaar';
              return null;
            },
          ),
          const SizedBox(height: 12),
          if (!_otpSent) ...[
            ElevatedButton.icon(
              onPressed: _loading ? null : _sendOtp,
              icon: _loading
                  ? const SizedBox(
                      width: 16,
                      height: 16,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Icon(Icons.send),
              label: Text(_loading ? 'Sending...' : 'Send OTP'),
            ),
            const SizedBox(height: 10),
            Text(
              'Or use Aadhaar QR at an authorised kiosk',
              style: TextStyle(color: Colors.grey.shade400, fontSize: 12),
            ),
          ] else ...[
            TextFormField(
              controller: _otpController,
              keyboardType: TextInputType.number,
              inputFormatters: [
                FilteringTextInputFormatter.digitsOnly,
                LengthLimitingTextInputFormatter(6),
              ],
              decoration: const InputDecoration(
                labelText: 'OTP',
                hintText: 'Enter OTP',
                prefixIcon: Icon(Icons.lock),
              ),
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: ElevatedButton(
                    onPressed: _loading ? null : _verifyOtpAndLogin,
                    child: _loading
                        ? const SizedBox(
                            width: 16,
                            height: 16,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          )
                        : const Text('Verify & Continue'),
                  ),
                ),
                const SizedBox(width: 10),
                TextButton(
                  onPressed: _loading
                      ? null
                      : () {
                          setState(() {
                            _otpSent = false;
                            _otpController.clear();
                          });
                        },
                  child: const Text('Use different Aadhaar'),
                ),
              ],
            ),
          ],
        ],
      ),
    );
  }
}

// *************** Inspector flow (email + password) ***************
class InspectorLogin extends StatefulWidget {
  @override
  State<InspectorLogin> createState() => _InspectorLoginState();
}

class _InspectorLoginState extends State<InspectorLogin> {
  final _formKey = GlobalKey<FormState>();
  final TextEditingController _emailC = TextEditingController();
  final TextEditingController _passC = TextEditingController();
  bool _loading = false;
  bool _obscure = true;

  @override
  void dispose() {
    _emailC.dispose();
    _passC.dispose();
    super.dispose();
  }

  Future<void> _signInInspector() async {
    if (!_formKey.currentState!.validate()) return;
    setState(() => _loading = true);
    await Future.delayed(const Duration(milliseconds: 900));
    setState(() => _loading = false);

    // Navigate to Inspector Home
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(
        builder: (_) => InspectorHomePage(email: _emailC.text.trim()),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(
        children: [
          const SizedBox(height: 6),
          Text(
            'Inspector sign in',
            style: Theme.of(
              context,
            ).textTheme.titleMedium!.copyWith(fontWeight: FontWeight.w700),
          ),
          const SizedBox(height: 12),
          TextFormField(
            controller: _emailC,
            keyboardType: TextInputType.emailAddress,
            decoration: const InputDecoration(
              labelText: 'Official email',
              hintText: 'inspector@domain.gov',
              prefixIcon: Icon(Icons.email_outlined),
            ),
            validator: (v) {
              if (v == null || v.trim().isEmpty) return 'Please enter email';
              if (!RegExp(r'^[^@]+@[^@]+\.[^@]+').hasMatch(v.trim()))
                return 'Enter valid email';
              return null;
            },
          ),
          const SizedBox(height: 12),
          TextFormField(
            controller: _passC,
            obscureText: _obscure,
            decoration: InputDecoration(
              labelText: 'Password',
              prefixIcon: const Icon(Icons.lock_outline),
              suffixIcon: IconButton(
                onPressed: () => setState(() => _obscure = !_obscure),
                icon: Icon(_obscure ? Icons.visibility_off : Icons.visibility),
              ),
            ),
            validator: (v) {
              if (v == null || v.length < 6)
                return 'Password must be at least 6 characters';
              return null;
            },
          ),
          const SizedBox(height: 14),
          ElevatedButton(
            onPressed: _signInInspector,
            child: _loading
                ? const SizedBox(
                    width: 16,
                    height: 16,
                    child: CircularProgressIndicator(strokeWidth: 2),
                  )
                : const Text('Sign in'),
          ),
          const SizedBox(height: 8),
          TextButton(
            onPressed: () => ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(content: Text('Talk to admin to reset password')),
            ),
            child: const Text('Forgot password?'),
          ),
        ],
      ),
    );
  }
}

// *************** Home pages ***************
class PublicHomePage extends StatelessWidget {
  final String aadhaar;
  const PublicHomePage({required this.aadhaar, super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Public Home'),
      actions: [
    IconButton(
      icon: const Icon(Icons.logout),
      tooltip: 'Logout',
      onPressed: () {
        // Navigate back to login or main page
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (_) => const MyApp()), // or LoginShell()
        );
      },
    ),
  ],),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          
          Center(child: Text('Welcome,User',style: TextStyle(fontSize: 24,fontWeight: FontWeight.bold),)),
            SizedBox(height: 20,),

          ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (_) => PublicQRScannerPage()),
                );
              },
              child: Text("SCAN CODE"),
            ),
            SizedBox(height: 20,),

          ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (_) => PublicReportsScreen()),
                );
              },
              child: Text("View Reports"),
            ),
        ],
      ),
    );
  }
}

class InspectorHomePage extends StatelessWidget {
  final String email;
  const InspectorHomePage({required this.email, super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Inspector Home'),
      actions: [
    IconButton(
      icon: const Icon(Icons.logout),
      tooltip: 'Logout',
      onPressed: () {
        // Navigate back to login or main page
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (_) => const MyApp()), // or LoginShell()
        );
      },
    ),
  ],
      ),
      body: Center(
        child: Column(
           mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Text('Welcome, Inspector',style: TextStyle(fontSize: 24,fontWeight: FontWeight.bold),),
            SizedBox(height: 20,),
            ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (_) => QRScannerPage()),
                );
              },
              child: Text("SCAN CODE"),
            ),
            SizedBox(height: 20,),

          ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (_) => BatchesScreen()),
                );
              },
              child: Text("View Inspections"),
            ),
          ],
        ),
      ),
    );
  }
}
