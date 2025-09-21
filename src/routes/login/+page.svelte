<!-- @ts-nocheck -->
<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let username = '';
	let password = '';
	let error = '';
	let isLoading = false;

	// Hardcoded credentials
	const credentials = {
		admin: { password: 'admin123', role: 'admin' },
		inspector: { password: 'inspector123', role: 'inspector' },
		viewer: { password: 'viewer123', role: 'viewer' }
	};

	function handleLogin() {
		error = '';
		isLoading = true;

		// Simulate loading delay
		setTimeout(() => {
			const userKey = username.toLowerCase();
			const user =
				userKey === 'admin' || userKey === 'inspector' || userKey === 'viewer'
					? credentials[userKey]
					: null;

			if (user && user.password === password) {
				// Store user session
				sessionStorage.setItem(
					'user',
					JSON.stringify({
						username: username.toLowerCase(),
						role: user.role,
						loginTime: new Date().toISOString()
					})
				);

				// Redirect based on role
				goto(`/${user.role}-dashboard`);
			} else {
				error = 'Invalid username or password';
				isLoading = false;
			}
		}, 1000);
	}

	function handleKeyPress(/** @type {any} */ event) {
		if (event.key === 'Enter') {
			handleLogin();
		}
	}

	onMount(() => {
		// Check if already logged in
		const user = sessionStorage.getItem('user');
		if (user) {
			const userData = JSON.parse(user);
			goto(`/${userData.role}-dashboard`);
		}
	});
</script>

<svelte:head>
	<title>Login - Rail Ledger</title>
</svelte:head>

<div
	class="flex min-h-screen items-center justify-center bg-gradient-to-br from-gray-900 via-black to-gray-900 px-4"
>
	<!-- Background Pattern -->
	<div class="bg-pattern absolute inset-0 opacity-5"></div>

	<!-- Login Card -->
	<div class="relative w-full max-w-md">
		<!-- Glass morphism effect -->
		<div
			class="rounded-3xl border border-gray-700/40 bg-gray-900/40 p-8 shadow-2xl backdrop-blur-xl"
		>
			<!-- Header -->
			<div class="mb-8 text-center">
				<h1 class="mb-2 text-3xl font-bold text-white">Rail Ledger</h1>
				<p class="text-gray-400">Sign in to your account</p>
			</div>

			<!-- Login Form -->
			<form on:submit|preventDefault={handleLogin} class="space-y-6">
				<!-- Username Field -->
				<div>
					<label for="username" class="mb-2 block text-sm font-medium text-gray-300">
						Username
					</label>
					<div class="relative">
						<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<svg
								class="h-5 w-5 text-gray-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
								></path>
							</svg>
						</div>
						<input
							id="username"
							type="text"
							bind:value={username}
							on:keypress={handleKeyPress}
							class="w-full rounded-xl border border-gray-600/50 bg-gray-800/50 py-3 pr-4 pl-10 text-white placeholder-gray-400 transition-all duration-300 focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/50 focus:outline-none"
							placeholder="Enter username"
							required
						/>
					</div>
				</div>

				<!-- Password Field -->
				<div>
					<label for="password" class="mb-2 block text-sm font-medium text-gray-300">
						Password
					</label>
					<div class="relative">
						<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<svg
								class="h-5 w-5 text-gray-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
								></path>
							</svg>
						</div>
						<input
							id="password"
							type="password"
							bind:value={password}
							on:keypress={handleKeyPress}
							class="w-full rounded-xl border border-gray-600/50 bg-gray-800/50 py-3 pr-4 pl-10 text-white placeholder-gray-400 transition-all duration-300 focus:border-blue-500/50 focus:ring-2 focus:ring-blue-500/50 focus:outline-none"
							placeholder="Enter password"
							required
						/>
					</div>
				</div>

				<!-- Error Message -->
				{#if error}
					<div
						class="rounded-xl border border-red-500/30 bg-red-900/20 px-4 py-3 text-sm text-red-300"
					>
						{error}
					</div>
				{/if}

				<!-- Login Button -->
				<button
					type="submit"
					disabled={isLoading}
					class="w-full rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-3 font-bold text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30 disabled:scale-100 disabled:cursor-not-allowed disabled:bg-gray-600/60"
				>
					{#if isLoading}
						<div class="flex items-center justify-center">
							<svg
								class="mr-3 -ml-1 h-5 w-5 animate-spin text-white"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
							>
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
							Signing in...
						</div>
					{:else}
						Sign In
					{/if}
				</button>
			</form>

			<!-- Demo Credentials -->
			<div class="mt-8 rounded-xl border border-gray-700/30 bg-gray-800/30 p-4">
				<h3 class="mb-3 text-sm font-medium text-gray-300">Demo Credentials:</h3>
				<div class="space-y-2 text-xs text-gray-400">
					<div class="flex justify-between">
						<span>Admin:</span>
						<span class="text-blue-300">admin / admin123</span>
					</div>
					<div class="flex justify-between">
						<span>Inspector:</span>
						<span class="text-green-300">inspector / inspector123</span>
					</div>
					<div class="flex justify-between">
						<span>Viewer:</span>
						<span class="text-purple-300">viewer / viewer123</span>
					</div>
				</div>
			</div>

			<!-- Back to Home -->
			<div class="mt-6 text-center">
				<a href="/" class="text-sm text-gray-400 transition-colors duration-300 hover:text-white">
					← Back to Home
				</a>
			</div>
		</div>
	</div>
</div>

<style>
	.bg-pattern {
		background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23ffffff' fill-opacity='0.05'%3E%3Ccircle cx='30' cy='30' r='2'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
		background-size: 60px 60px;
	}
</style>
