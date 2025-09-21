<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = null;
	let settings = {
		systemName: 'Rail Ledger',
		emailNotifications: true,
		systemAlerts: true,
		autoBackup: false,
		maintenanceMode: false
	};

	onMount(async () => {
		const userData = sessionStorage.getItem('user');
		if (!userData) {
			goto('/login');
			return;
		}

		user = JSON.parse(userData);
	});

	function saveSettings() {
		// In a real app, this would save to the backend
		alert('Settings saved successfully!');
	}
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 sm:mb-8">
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">System Settings</h1>
	</div>

	<div class="max-w-4xl">
		<!-- General Settings -->
		<div class="mb-6 rounded-lg border border-gray-600 bg-gray-800 p-4 sm:p-6">
			<h3 class="mb-4 text-xl font-bold text-white">General Settings</h3>
			<div class="space-y-4">
				<div>
					<label for="system-name" class="mb-2 block text-sm font-medium text-gray-300"
						>System Name</label
					>
					<input
						id="system-name"
						type="text"
						bind:value={settings.systemName}
						class="w-full rounded-lg border border-gray-600 bg-gray-700 px-4 py-3 text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					/>
				</div>
			</div>
		</div>

		<!-- Notification Settings -->
		<div class="mb-6 rounded-lg border border-gray-600 bg-gray-800 p-4 sm:p-6">
			<h3 class="mb-4 text-xl font-bold text-white">Notification Settings</h3>
			<div class="space-y-4">
				<label class="flex items-center">
					<input
						type="checkbox"
						bind:checked={settings.emailNotifications}
						class="mr-3 h-4 w-4 rounded border-gray-600 bg-gray-700 text-purple-600 focus:ring-purple-500"
					/>
					<span class="text-gray-300">Email notifications</span>
				</label>
				<label class="flex items-center">
					<input
						type="checkbox"
						bind:checked={settings.systemAlerts}
						class="mr-3 h-4 w-4 rounded border-gray-600 bg-gray-700 text-purple-600 focus:ring-purple-500"
					/>
					<span class="text-gray-300">System alerts</span>
				</label>
			</div>
		</div>

		<!-- System Settings -->
		<div class="mb-6 rounded-lg border border-gray-600 bg-gray-800 p-4 sm:p-6">
			<h3 class="mb-4 text-xl font-bold text-white">System Settings</h3>
			<div class="space-y-4">
				<label class="flex items-center">
					<input
						type="checkbox"
						bind:checked={settings.autoBackup}
						class="mr-3 h-4 w-4 rounded border-gray-600 bg-gray-700 text-purple-600 focus:ring-purple-500"
					/>
					<span class="text-gray-300">Automatic backup</span>
				</label>
				<label class="flex items-center">
					<input
						type="checkbox"
						bind:checked={settings.maintenanceMode}
						class="mr-3 h-4 w-4 rounded border-gray-600 bg-gray-700 text-purple-600 focus:ring-purple-500"
					/>
					<span class="text-gray-300">Maintenance mode</span>
				</label>
			</div>
		</div>

		<!-- Save Button -->
		<div class="flex justify-end">
			<button
				on:click={saveSettings}
				class="rounded-lg bg-purple-600 px-6 py-3 font-medium text-white transition-all duration-300 hover:bg-purple-700"
			>
				Save Settings
			</button>
		</div>
	</div>
</div>
