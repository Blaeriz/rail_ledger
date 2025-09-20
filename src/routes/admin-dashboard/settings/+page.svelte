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
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white mb-2">System Settings</h1>
  </div>
  
  <div class="max-w-4xl">
    <!-- General Settings -->
    <div class="bg-gray-800 border border-gray-600 rounded-lg p-4 sm:p-6 mb-6">
      <h3 class="text-xl font-bold text-white mb-4">General Settings</h3>
      <div class="space-y-4">
        <div>
          <label for="system-name" class="block text-gray-300 text-sm font-medium mb-2">System Name</label>
          <input 
            id="system-name" 
            type="text" 
            bind:value={settings.systemName}
            class="w-full bg-gray-700 border border-gray-600 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500"
          />
        </div>
      </div>
    </div>

    <!-- Notification Settings -->
    <div class="bg-gray-800 border border-gray-600 rounded-lg p-4 sm:p-6 mb-6">
      <h3 class="text-xl font-bold text-white mb-4">Notification Settings</h3>
      <div class="space-y-4">
        <label class="flex items-center">
          <input 
            type="checkbox" 
            bind:checked={settings.emailNotifications}
            class="mr-3 w-4 h-4 text-purple-600 bg-gray-700 border-gray-600 rounded focus:ring-purple-500"
          />
          <span class="text-gray-300">Email notifications</span>
        </label>
        <label class="flex items-center">
          <input 
            type="checkbox" 
            bind:checked={settings.systemAlerts}
            class="mr-3 w-4 h-4 text-purple-600 bg-gray-700 border-gray-600 rounded focus:ring-purple-500"
          />
          <span class="text-gray-300">System alerts</span>
        </label>
      </div>
    </div>

    <!-- System Settings -->
    <div class="bg-gray-800 border border-gray-600 rounded-lg p-4 sm:p-6 mb-6">
      <h3 class="text-xl font-bold text-white mb-4">System Settings</h3>
      <div class="space-y-4">
        <label class="flex items-center">
          <input 
            type="checkbox" 
            bind:checked={settings.autoBackup}
            class="mr-3 w-4 h-4 text-purple-600 bg-gray-700 border-gray-600 rounded focus:ring-purple-500"
          />
          <span class="text-gray-300">Automatic backup</span>
        </label>
        <label class="flex items-center">
          <input 
            type="checkbox" 
            bind:checked={settings.maintenanceMode}
            class="mr-3 w-4 h-4 text-purple-600 bg-gray-700 border-gray-600 rounded focus:ring-purple-500"
          />
          <span class="text-gray-300">Maintenance mode</span>
        </label>
      </div>
    </div>

    <!-- Save Button -->
    <div class="flex justify-end">
      <button 
        on:click={saveSettings}
        class="bg-purple-600 hover:bg-purple-700 text-white px-6 py-3 rounded-lg font-medium transition-all duration-300"
      >
        Save Settings
      </button>
    </div>
  </div>
</div>