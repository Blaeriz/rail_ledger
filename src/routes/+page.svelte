<script>
  const roles = [
    {
      id: 'admin',
      title: 'Admin',
      description:
        'Oversee procurement, supply chain, and AI-driven quality monitoring of track fittings with UDM + TMS integration.                            ' ,
      icon: '⚙️',
      color: 'from-blue-500 to-blue-600'
    },
    {
      id: 'inspector',
      title: 'Inspector',
      description:
        'Scan QR codes on elastic rail clips, rail pads, liners, and sleepers to access supply history, warranty, and inspection records.',
      icon: '🔍',
      color: 'from-green-500 to-green-600'
    },
    {
      id: 'vendor',
      title: 'Vendor',
      description:
        'Register supplies, imprint laser-based QR codes, and sync shipment + warranty details with UDM and TMS portals.',
      icon: '💼',
      color: 'from-purple-500 to-purple-600'
    }
  ];
</script>

<div class="min-h-screen bg-gray-900 text-gray-100">
  <!-- Header -->
  <header class="border-b border-gray-800 bg-gray-800/50 backdrop-blur-sm">
    <div class="container mx-auto px-4 py-6 flex items-center justify-between">
      <h1 class="text-2xl font-bold text-white">Rail Ledger</h1>
      <nav class="hidden md:flex space-x-6">
        <button class="text-gray-400 hover:text-white">Support</button>
        <button class="text-gray-400 hover:text-white">Contact</button>
      </nav>
    </div>
  </header>

  <!-- Intro -->
  <main class="container mx-auto px-4 py-12">
    <div class="text-center mb-12">
      <h2 class="text-4xl font-bold text-white mb-4">QR-enabled Track Fitting Management</h2>
      <p class="text-xl text-gray-400 max-w-2xl mx-auto">
        A unified platform for Indian Railways to identify, track, and monitor elastic rail clips, rail pads, liners, and sleepers using laser-based QR codes integrated with UDM & TMS.
      </p>
    </div>

    <!-- Role Selection -->
    <div class="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
      {#each roles as role}
        <div class="group relative">
          <div class="bg-gray-800 border border-gray-700 rounded-lg p-8 text-center hover:shadow-lg transition-all hover:-translate-y-1">
           
            <h3 class="text-2xl font-bold mb-4">{role.title}</h3>
            <p class="text-gray-400 mb-8">{role.description}</p>
            <button class="w-full bg-indigo-600 hover:bg-indigo-500 text-white font-semibold py-3 px-6 rounded-lg">
              Access as {role.title}
            </button>
          </div>
          <div class="absolute inset-0 bg-gradient-to-r {role.color} rounded-lg opacity-0 group-hover:opacity-20 transition-opacity -z-10"></div>
        </div>
      {/each}
    </div>

    <!-- Extra Info -->
    <div class="mt-16 text-center bg-gray-800 rounded-lg p-8 max-w-4xl mx-auto">
      <h3 class="text-xl font-semibold mb-4 text-white">Why Rail Ledger?</h3>
      <p class="text-gray-400 mb-6">
        By combining hardware (laser QR codes) and software (AI + UDM + TMS integration), Rail Ledger ensures traceability, performance monitoring, and safety of fittings used across Indian Railways.
      </p>
      <button class="bg-indigo-600 hover:bg-indigo-500 text-white px-6 py-2 rounded-lg">Learn More</button>
    </div>
  </main>

  <!-- Footer -->
  <footer class="border-t border-gray-700 bg-gray-800/50 mt-20">
    <div class="container mx-auto px-4 py-8 flex flex-col md:flex-row justify-between items-center">
      <p class="text-sm text-gray-400">© 2025 Rail Ledger. All rights reserved.</p>
      <div class="flex space-x-6 text-sm">
        <button class="text-gray-400 hover:text-white">Privacy</button>
        <button class="text-gray-400 hover:text-white">Terms</button>
      </div>
    </div>
  </footer>
</div>
