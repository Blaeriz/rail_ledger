<!-- @ts-nocheck -->
<script>
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Icon from '$lib/components/Icon.svelte';

	let user = /** @type {any} */ (null);
	let isLoading = true;
	let isScrolled = false;
	let showProfile = false;

	// Navigation items for viewer with professional icons
	const navItems = [
		{ id: 'analytics', label: 'Analytics', href: '/viewer-dashboard/analytics', icon: 'analytics' },
		{ id: 'overview', label: 'Overview', href: '/viewer-dashboard/overview', icon: 'home' },
		{ id: 'vendors', label: 'Vendors', href: '/viewer-dashboard/vendors', icon: 'building' },
		{ id: 'batches', label: 'Batches', href: '/viewer-dashboard/batches', icon: 'package' },
		{ id: 'reports', label: 'Reports', href: '/viewer-dashboard/reports', icon: 'clipboard' },
		{ id: 'tickets', label: 'Tickets', href: '/viewer-dashboard/tickets', icon: 'ticket' }
	];

	function handleScroll() {
		if (typeof window !== 'undefined') {
			isScrolled = window.scrollY > 10;
		}
	}

	onMount(async () => {
		// Check authentication
		const userData = sessionStorage.getItem('user');
		if (!userData) {
			goto('/login');
			return;
		}

		user = JSON.parse(userData);
		isLoading = false;

		// Add scroll listener
		if (typeof window !== 'undefined') {
			window.addEventListener('scroll', handleScroll);
			window.addEventListener('click', handleClickOutside);
		}
	});

	onDestroy(() => {
		if (typeof window !== 'undefined') {
			window.removeEventListener('scroll', handleScroll);
			window.removeEventListener('click', handleClickOutside);
		}
	});

	function logout() {
		sessionStorage.removeItem('user');
		goto('/login');
	}

	function isActive(/** @type {string} */ href) {
		return $page.url.pathname === href;
	}

	function toggleProfile() {
		showProfile = !showProfile;
	}

	function closeProfile() {
		showProfile = false;
	}

	function handleClickOutside(/** @type {any} */ event) {
		if (showProfile && !event.target.closest('.profile-dropdown')) {
			closeProfile();
		}
	}
</script>

<svelte:head>
	<title>Viewer Dashboard - Rail Ledger</title>
</svelte:head>

<div class="min-h-screen bg-black">
	<!-- Top Navbar - Fixed Position -->
	<nav
		class="fixed top-0 right-0 left-0 z-50 transition-all duration-300 {isScrolled
			? 'border-b border-gray-800/50 bg-black/90 backdrop-blur-md'
			: 'border-b border-gray-800 bg-black'} px-4 py-3 sm:px-6 sm:py-4"
	>
		<div class="flex items-center justify-between">
			<!-- Left side - Logo -->
			<div class="flex items-center">
				<h1 class="text-xl font-bold text-white sm:text-2xl">Rail Ledger</h1>
			</div>

			<!-- Right side - User info and actions -->
			<div class="relative flex items-center space-x-2 sm:space-x-4">
				<span class="hidden text-xs text-gray-300 sm:block sm:text-sm">{user?.username}</span>
				<div class="profile-dropdown relative">
					<button
						on:click={toggleProfile}
						class="rounded-lg border border-purple-500/30 bg-gray-800 px-4 py-2 text-xs text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30 sm:text-sm"
					>
						PROFILE
					</button>

					<!-- Profile Dropdown -->
					{#if showProfile}
						<div
							class="animate-fade-in absolute top-full right-0 z-50 mt-2 w-64 rounded-lg border border-gray-700 bg-gray-800 shadow-2xl shadow-black/50"
						>
							<div class="p-4">
								<div class="mb-4 flex items-center space-x-3">
									<div
										class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-green-500 to-emerald-500"
									>
										<span class="text-sm font-bold text-white"
											>{user?.username?.charAt(0)?.toUpperCase()}</span
										>
									</div>
									<div>
										<h3 class="text-sm font-semibold text-white">{user?.username}</h3>
										<p class="text-xs text-gray-400">viewer</p>
									</div>
								</div>

								<div class="border-t border-gray-700 pt-3">
									<div class="space-y-2">
										<div class="flex justify-between text-xs">
											<span class="text-gray-400">User ID:</span>
											<span class="text-white">{user?.user_id || 'N/A'}</span>
										</div>
										<div class="flex justify-between text-xs">
											<span class="text-gray-400">Role:</span>
											<span class="text-green-400">Viewer</span>
										</div>
										<div class="flex justify-between text-xs">
											<span class="text-gray-400">Status:</span>
											<span class="text-green-400">Active</span>
										</div>
									</div>
								</div>

								<div class="mt-3 border-t border-gray-700 pt-3">
									<button
										on:click={logout}
										class="w-full rounded px-3 py-2 text-left text-xs text-red-400 transition-colors duration-200 hover:bg-red-500/10 hover:text-red-300"
									>
										Sign Out
									</button>
								</div>
							</div>
						</div>
					{/if}
				</div>

				<button
					on:click={logout}
					class="rounded-full border border-red-600 bg-red-800 px-4 py-2 text-xs font-bold tracking-wide text-white shadow-lg shadow-red-500/30 transition-all duration-300 hover:border-red-500 hover:bg-red-700 hover:shadow-xl hover:shadow-red-500/40 sm:text-sm"
				>
					LOGOUT
				</button>
			</div>
		</div>
	</nav>

	<div class="flex pt-16">
		<!-- Sidebar - Fixed Position -->
		<aside
			class="fixed top-16 left-0 z-40 h-screen w-48 overflow-y-auto border-r border-gray-800 bg-black sm:w-64"
		>
			<nav class="space-y-1 p-3 sm:space-y-2 sm:p-4">
				{#each navItems as item}
					<a
						href={item.href}
						class="group flex w-full items-center space-x-2 rounded-lg px-3 py-2 transition-all duration-300 sm:space-x-3 sm:px-4 sm:py-3 {isActive(
							item.href
						)
							? 'border border-gray-500 bg-gray-700 text-white shadow-lg shadow-gray-500/10'
							: 'border border-transparent text-gray-300 hover:border-gray-500/30 hover:bg-gray-700/20 hover:text-white hover:shadow-lg hover:shadow-gray-500/5 active:border-gray-500/40 active:bg-gray-700/30 active:shadow-xl active:shadow-gray-500/10'}"
					>
						<Icon
							name={item.icon}
							size="w-5 h-5 sm:w-6 sm:h-6"
							className="transition-transform duration-300 group-hover:scale-110"
						/>
						<span class="text-sm font-medium sm:text-base">{item.label}</span>
						{#if isActive(item.href)}
							<div class="ml-auto h-1.5 w-1.5 rounded-full bg-gray-400 sm:h-2 sm:w-2"></div>
						{/if}
					</a>
				{/each}
			</nav>
		</aside>

		<!-- Main Content -->
		<main class="ml-48 min-h-screen flex-1 overflow-x-auto bg-black sm:ml-64">
			{#if isLoading}
				<div class="flex h-64 items-center justify-center">
					<div class="h-12 w-12 animate-spin rounded-full border-b-2 border-purple-500"></div>
				</div>
			{:else}
				<slot />
			{/if}
		</main>
	</div>
</div>
