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

<div class="relative min-h-screen overflow-hidden bg-black text-white">
	<!-- Background -->
	<div class="absolute inset-0 bg-black">
		<div class="absolute inset-0 bg-black"></div>
		<div class="bg-pattern absolute inset-0 opacity-10"></div>
	</div>

	<!-- Professional Header -->
	<header
		class="relative border-b border-gray-800/40 bg-black/20 shadow-lg shadow-black/20 backdrop-blur-2xl"
	>
		<div class="container mx-auto flex items-center justify-between px-8 py-6">
			<div class="flex items-center space-x-4">
				<div>
					<h1
						class="bg-gradient-to-r from-white via-blue-100 to-indigo-200 bg-clip-text text-3xl font-bold text-transparent drop-shadow-lg"
						style="text-shadow: 0 0 20px rgba(255, 255, 255, 0.1), 0 0 40px rgba(59, 130, 246, 0.05);"
					>
						Rail Ledger
					</h1>
					<p class="text-xs font-medium tracking-wider text-gray-400 uppercase">
						Enterprise Management System
					</p>
				</div>
			</div>
			<nav class="hidden items-center space-x-6 md:flex">
				<button class="text-gray-300 transition-colors duration-300 hover:text-white"
					>Documentation</button
				>
				<button class="text-gray-300 transition-colors duration-300 hover:text-white"
					>Support</button
				>
				<button
					class="rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-2 text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30"
				>
					Get Started
				</button>
			</nav>
		</div>
	</header>

	<!-- Professional Hero Section -->
	<main class="relative container mx-auto bg-black px-8 py-16">
		<div class="mb-16 text-center">
			<div
				class="mb-12 inline-flex items-center rounded-full border border-blue-500/20 bg-gradient-to-r from-blue-500/10 to-purple-500/10 px-6 py-3 text-sm font-semibold tracking-wide text-blue-300"
			>
				<svg
					class="mr-3 h-4 w-4 text-blue-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M13 10V3L4 14h7v7l9-11h-7z"
					></path>
				</svg>
				Enterprise Railway Management Platform
			</div>
			<h2 class="mb-8 text-6xl leading-tight font-bold">
				<span
					class="bg-gradient-to-r from-white via-blue-100 to-indigo-200 bg-clip-text text-transparent"
					>QR-Enabled Track Fitting</span
				>
				<span class="block text-blue-300">Management System</span>
			</h2>
			<p class="mx-auto mb-8 max-w-4xl text-xl leading-relaxed text-gray-300">
				A comprehensive enterprise platform for Indian Railways to identify, track, and monitor
				elastic rail clips, rail pads, liners, and sleepers using advanced laser-based QR codes with
				seamless UDM & TMS integration.
			</p>
			<div class="flex justify-center gap-4">
				<a
					href="/login"
					class="rounded-xl bg-gray-700/60 px-8 py-3 font-bold text-white transition-colors duration-300 hover:bg-purple-600/80"
				>
					Login
				</a>
				<a
					href="/AI-Reports"
					class="rounded-xl bg-gray-700/60 px-8 py-3 font-bold text-white transition-colors duration-300 hover:bg-blue-600/80"
				>
					View AI Reports
				</a>
			</div>
		</div>

		<!-- Professional Role Selection -->
		<div class="mx-auto mb-20 grid max-w-7xl gap-8 md:grid-cols-3">
			{#each roles as role}
				<div class="group relative">
					<div
						class="border border-gray-800/40 bg-black/90 backdrop-blur-2xl {role.hoverColor} rounded-2xl p-6 text-center transition-all duration-300 hover:-translate-y-2 hover:border-purple-500/30 hover:shadow-xl"
					>
						<div
							class="mx-auto h-12 w-12 {role.bgColor} mb-4 flex items-center justify-center rounded-xl transition-transform duration-300 group-hover:scale-110"
						>
							{@html role.icon}
						</div>
						<h3 class="mb-3 text-xl font-bold text-white">{role.title}</h3>
						<p class="mb-6 text-sm leading-relaxed text-gray-400">{role.description}</p>
						<button
							class="w-full rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-3 font-bold text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30"
						>
							Access as {role.title}
						</button>
					</div>
				</div>
			{/each}
		</div>

		<!-- Features Section -->
		<div
			class="mx-auto max-w-6xl rounded-2xl border border-gray-800/40 bg-black/95 p-12 backdrop-blur-2xl"
		>
			<div class="mb-12 text-center">
				<h3
					class="mb-4 bg-gradient-to-r from-white via-blue-100 to-indigo-200 bg-clip-text text-4xl font-bold text-transparent"
				>
					Why Choose Rail Ledger?
				</h3>
				<p class="text-lg text-gray-300">Enterprise-grade technology meets railway expertise</p>
			</div>

			<div class="mb-12 grid gap-8 md:grid-cols-3">
				<div class="group text-center">
					<div
						class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gray-700/40 transition-all duration-300 group-hover:scale-110 group-hover:bg-purple-600/40"
					>
						<div class="h-6 w-6 rounded-lg bg-white/30"></div>
					</div>
					<h4 class="mb-3 text-xl font-bold text-white">UDM Integration</h4>
					<p class="text-sm leading-relaxed text-gray-400">
						Seamless integration with Unified Data Management systems for comprehensive data flow
					</p>
				</div>
				<div class="group text-center">
					<div
						class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gray-700/40 transition-all duration-300 group-hover:scale-110 group-hover:bg-purple-600/40"
					>
						<div class="h-6 w-6 rounded-lg bg-white/30"></div>
					</div>
					<h4 class="mb-3 text-xl font-bold text-white">AI Monitoring</h4>
					<p class="text-sm leading-relaxed text-gray-400">
						Advanced AI-driven quality monitoring and predictive maintenance capabilities
					</p>
				</div>
				<div class="group text-center">
					<div
						class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-gray-700/40 transition-all duration-300 group-hover:scale-110 group-hover:bg-purple-600/40"
					>
						<div class="h-6 w-6 rounded-lg bg-white/30"></div>
					</div>
					<h4 class="mb-3 text-xl font-bold text-white">TMS Sync</h4>
					<p class="text-sm leading-relaxed text-gray-400">
						Real-time synchronization with Transport Management Systems for optimal efficiency
					</p>
				</div>
			</div>

			<div class="text-center">
				<p class="mx-auto mb-8 max-w-3xl text-lg text-gray-300">
					By combining cutting-edge hardware (laser QR codes) and advanced software (AI + UDM + TMS
					integration), Rail Ledger ensures complete traceability, performance monitoring, and
					safety of fittings used across Indian Railways infrastructure.
				</p>
				<button
					class="rounded-xl bg-blue-600 px-8 py-3 font-bold text-white transition-colors duration-300 hover:bg-blue-700"
				>
					Learn More
				</button>
			</div>
		</div>
	</main>

	<!-- Footer -->
	<footer class="relative mt-20 border-t border-gray-700/40 bg-gray-900/20 backdrop-blur-2xl">
		<div class="container mx-auto px-8 py-12">
			<div class="mb-8 grid gap-8 md:grid-cols-3">
				<div>
					<h4 class="mb-4 text-xl font-bold text-white">Rail Ledger</h4>
					<p class="max-w-md text-sm text-gray-300">
						Revolutionizing railway infrastructure management through advanced QR technology and
						AI-driven insights.
					</p>
				</div>

				<div>
					<h5 class="mb-4 text-lg font-bold text-white">Quick Links</h5>
					<div class="space-y-2">
						<button
							class="block text-sm text-gray-400 transition-colors duration-300 hover:text-white"
							>Documentation</button
						>
						<button
							class="block text-sm text-gray-400 transition-colors duration-300 hover:text-white"
							>API Reference</button
						>
						<button
							class="block text-sm text-gray-400 transition-colors duration-300 hover:text-white"
							>Support Center</button
						>
					</div>
				</div>

				<div>
					<h5 class="mb-4 text-lg font-bold text-white">Legal</h5>
					<div class="space-y-2">
						<button
							class="block text-sm text-gray-400 transition-colors duration-300 hover:text-white"
							>Privacy Policy</button
						>
						<button
							class="block text-sm text-gray-400 transition-colors duration-300 hover:text-white"
							>Terms of Service</button
						>
						<button
							class="block text-sm text-gray-400 transition-colors duration-300 hover:text-white"
							>Compliance</button
						>
					</div>
				</div>
			</div>

			<div class="border-t border-gray-800/50 pt-6">
				<p class="text-center text-sm text-gray-400">© 2025 Rail Ledger. All rights reserved.</p>
			</div>
		</div>
	</footer>
</div>
