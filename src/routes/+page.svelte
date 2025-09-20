<script>
  const roles = [
    {
      id: 'admin',
      title: 'Admin',
      description:
        'Oversee procurement, supply chain, and AI-driven quality monitoring of track fittings with UDM + TMS integration.',
      icon: `<svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
      </svg>`,
      color: 'from-blue-500 to-blue-600',
      bgColor: 'bg-blue-500/10',
      borderColor: 'border-blue-500/20',
      hoverColor: 'hover:border-blue-500/30'
    },
    {
      id: 'inspector',
      title: 'Inspector',
      description:
        'Scan QR codes on elastic rail clips, rail pads, liners, and sleepers to access supply history, warranty, and inspection records.',
      icon: `<svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
      </svg>`,
      color: 'from-green-500 to-green-600',
      bgColor: 'bg-green-500/10',
      borderColor: 'border-green-500/20',
      hoverColor: 'hover:border-green-500/30'
    },
    {
      id: 'vendor',
      title: 'Vendor',
      description:
        'Register supplies, imprint laser-based QR codes, and sync shipment + warranty details with UDM and TMS portals.',
      icon: `<svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
        <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm2 6a2 2 0 114 0 2 2 0 01-4 0zm8 0a2 2 0 114 0 2 2 0 01-4 0z" clip-rule="evenodd" />
      </svg>`,
      color: 'from-purple-500 to-purple-600',
      bgColor: 'bg-purple-500/10',
      borderColor: 'border-purple-500/20',
      hoverColor: 'hover:border-purple-500/30'
    }
  ];
</script>

<div class="min-h-screen bg-black text-white relative overflow-hidden">
  <!-- Background -->
  <div class="absolute inset-0 bg-black">
    <div class="absolute inset-0 bg-black"></div>
    <div class="absolute inset-0 opacity-10 bg-pattern"></div>
  </div>
  
  <!-- Professional Header -->
  <header class="relative border-b border-gray-800/40 bg-black/20 backdrop-blur-2xl shadow-lg shadow-black/20">
    <div class="container mx-auto px-8 py-6 flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <div>
          <h1 class="text-3xl font-bold bg-gradient-to-r from-white via-blue-100 to-indigo-200 bg-clip-text text-transparent drop-shadow-lg" style="text-shadow: 0 0 20px rgba(255, 255, 255, 0.1), 0 0 40px rgba(59, 130, 246, 0.05);">Rail Ledger</h1>
          <p class="text-xs text-gray-400 font-medium tracking-wider uppercase">Enterprise Management System</p>
        </div>
      </div>
      <nav class="hidden md:flex items-center space-x-6">
        <button class="text-gray-300 hover:text-white transition-colors duration-300">Documentation</button>
        <button class="text-gray-300 hover:text-white transition-colors duration-300">Support</button>
        <button class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-2 rounded-lg transition-all duration-300 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30">
          Get Started
        </button>
      </nav>
    </div>
  </header>

  <!-- Professional Hero Section -->
  <main class="relative container mx-auto px-8 py-16 bg-black">
    <div class="text-center mb-16">
      <div class="inline-flex items-center px-6 py-3 bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-blue-500/20 rounded-full text-blue-300 text-sm font-semibold mb-12 tracking-wide">
        <svg class="w-4 h-4 mr-3 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
        </svg>
        Enterprise Railway Management Platform
      </div>
      <h2 class="text-6xl font-bold mb-8 leading-tight">
        <span class="bg-gradient-to-r from-white via-blue-100 to-indigo-200 bg-clip-text text-transparent">QR-Enabled Track Fitting</span>
        <span class="block text-blue-300">Management System</span>
      </h2>
      <p class="text-xl text-gray-300 max-w-4xl mx-auto leading-relaxed mb-8">
        A comprehensive enterprise platform for Indian Railways to identify, track, and monitor elastic rail clips, rail pads, liners, and sleepers using advanced laser-based QR codes with seamless UDM & TMS integration.
      </p>
      <div class="flex justify-center gap-4">
        <a href="/login" class="bg-gray-700/60 hover:bg-purple-600/80 text-white px-8 py-3 rounded-xl font-bold transition-colors duration-300">
          Login
        </a>
        <a href="/AI-Reports" class="bg-gray-700/60 hover:bg-blue-600/80 text-white px-8 py-3 rounded-xl font-bold transition-colors duration-300">
          View AI Reports
        </a>
      </div>
    </div>

    <!-- Professional Role Selection -->
    <div class="grid md:grid-cols-3 gap-8 max-w-7xl mx-auto mb-20">
      {#each roles as role}
        <div class="group relative">
          <div class="bg-black/90 backdrop-blur-2xl border border-gray-800/40 {role.hoverColor} rounded-2xl p-6 text-center transition-all duration-300 hover:shadow-xl hover:-translate-y-2 hover:border-purple-500/30">
            <div class="w-12 h-12 mx-auto {role.bgColor} rounded-xl flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
              {@html role.icon}
            </div>
            <h3 class="text-xl font-bold mb-3 text-white">{role.title}</h3>
            <p class="text-gray-400 mb-6 leading-relaxed text-sm">{role.description}</p>
            <button class="w-full bg-gray-800 hover:bg-gray-700 text-white font-bold py-3 px-6 rounded-lg transition-all duration-300 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30">
              Access as {role.title}
            </button>
          </div>
        </div>
      {/each}
    </div>

    <!-- Features Section -->
    <div class="bg-black/95 backdrop-blur-2xl rounded-2xl p-12 max-w-6xl mx-auto border border-gray-800/40">
      <div class="text-center mb-12">
        <h3 class="text-4xl font-bold mb-4 bg-gradient-to-r from-white via-blue-100 to-indigo-200 bg-clip-text text-transparent">Why Choose Rail Ledger?</h3>
        <p class="text-gray-300 text-lg">Enterprise-grade technology meets railway expertise</p>
      </div>
      
      <div class="grid md:grid-cols-3 gap-8 mb-12">
        <div class="text-center group">
          <div class="w-12 h-12 bg-gray-700/40 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:scale-110 group-hover:bg-purple-600/40 transition-all duration-300">
            <div class="w-6 h-6 bg-white/30 rounded-lg"></div>
          </div>
          <h4 class="text-xl font-bold text-white mb-3">UDM Integration</h4>
          <p class="text-gray-400 text-sm leading-relaxed">Seamless integration with Unified Data Management systems for comprehensive data flow</p>
        </div>
        <div class="text-center group">
          <div class="w-12 h-12 bg-gray-700/40 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:scale-110 group-hover:bg-purple-600/40 transition-all duration-300">
            <div class="w-6 h-6 bg-white/30 rounded-lg"></div>
          </div>
          <h4 class="text-xl font-bold text-white mb-3">AI Monitoring</h4>
          <p class="text-gray-400 text-sm leading-relaxed">Advanced AI-driven quality monitoring and predictive maintenance capabilities</p>
        </div>
        <div class="text-center group">
          <div class="w-12 h-12 bg-gray-700/40 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:scale-110 group-hover:bg-purple-600/40 transition-all duration-300">
            <div class="w-6 h-6 bg-white/30 rounded-lg"></div>
          </div>
          <h4 class="text-xl font-bold text-white mb-3">TMS Sync</h4>
          <p class="text-gray-400 text-sm leading-relaxed">Real-time synchronization with Transport Management Systems for optimal efficiency</p>
        </div>
      </div>
      
      <div class="text-center">
        <p class="text-gray-300 mb-8 text-lg max-w-3xl mx-auto">
          By combining cutting-edge hardware (laser QR codes) and advanced software (AI + UDM + TMS integration), Rail Ledger ensures complete traceability, performance monitoring, and safety of fittings used across Indian Railways infrastructure.
        </p>
        <button class="bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 rounded-xl font-bold transition-colors duration-300">
          Learn More
        </button>
      </div>
    </div>
  </main>

  <!-- Footer -->
  <footer class="relative border-t border-gray-700/40 bg-gray-900/20 backdrop-blur-2xl mt-20">
    <div class="container mx-auto px-8 py-12">
      <div class="grid md:grid-cols-3 gap-8 mb-8">
        <div>
          <h4 class="text-xl font-bold text-white mb-4">Rail Ledger</h4>
          <p class="text-gray-300 text-sm max-w-md">
            Revolutionizing railway infrastructure management through advanced QR technology and AI-driven insights.
          </p>
        </div>
        
        <div>
          <h5 class="text-lg font-bold text-white mb-4">Quick Links</h5>
          <div class="space-y-2">
            <button class="block text-gray-400 hover:text-white transition-colors duration-300 text-sm">Documentation</button>
            <button class="block text-gray-400 hover:text-white transition-colors duration-300 text-sm">API Reference</button>
            <button class="block text-gray-400 hover:text-white transition-colors duration-300 text-sm">Support Center</button>
          </div>
        </div>
        
        <div>
          <h5 class="text-lg font-bold text-white mb-4">Legal</h5>
          <div class="space-y-2">
            <button class="block text-gray-400 hover:text-white transition-colors duration-300 text-sm">Privacy Policy</button>
            <button class="block text-gray-400 hover:text-white transition-colors duration-300 text-sm">Terms of Service</button>
            <button class="block text-gray-400 hover:text-white transition-colors duration-300 text-sm">Compliance</button>
          </div>
        </div>
      </div>
      
      <div class="border-t border-gray-800/50 pt-6">
        <p class="text-gray-400 text-sm text-center">© 2025 Rail Ledger. All rights reserved.</p>
      </div>
    </div>
  </footer>
</div>