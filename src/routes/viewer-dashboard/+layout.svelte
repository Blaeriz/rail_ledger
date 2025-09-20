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
  <nav class="fixed top-0 left-0 right-0 z-50 transition-all duration-300 {isScrolled ? 'bg-black/90 backdrop-blur-md border-b border-gray-800/50' : 'bg-black border-b border-gray-800'} px-4 sm:px-6 py-3 sm:py-4">
    <div class="flex items-center justify-between">
      <!-- Left side - Logo -->
      <div class="flex items-center">
        <h1 class="text-xl sm:text-2xl font-bold text-white">Rail Ledger</h1>
      </div>
      
      <!-- Right side - User info and actions -->
      <div class="flex items-center space-x-2 sm:space-x-4 relative">
        <span class="text-gray-300 text-xs sm:text-sm hidden sm:block">{user?.username}</span>
        <div class="relative profile-dropdown">
          <button 
            on:click={toggleProfile}
            class="bg-gray-800 hover:bg-gray-700 text-white px-4 py-2 rounded-lg text-xs sm:text-sm transition-all duration-300 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30"
          >
            PROFILE
          </button>
          
          <!-- Profile Dropdown -->
          {#if showProfile}
            <div class="absolute right-0 top-full mt-2 w-64 bg-gray-800 border border-gray-700 rounded-lg shadow-2xl shadow-black/50 z-50 animate-fade-in">
              <div class="p-4">
                <div class="flex items-center space-x-3 mb-4">
                  <div class="w-10 h-10 bg-gradient-to-br from-green-500 to-emerald-500 rounded-full flex items-center justify-center">
                    <span class="text-white font-bold text-sm">{user?.username?.charAt(0)?.toUpperCase()}</span>
                  </div>
                  <div>
                    <h3 class="text-white font-semibold text-sm">{user?.username}</h3>
                    <p class="text-gray-400 text-xs">viewer</p>
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
                
                <div class="border-t border-gray-700 pt-3 mt-3">
                  <button 
                    on:click={logout}
                    class="w-full text-left px-3 py-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded text-xs transition-colors duration-200"
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
          class="bg-red-800 hover:bg-red-700 text-white px-4 py-2 rounded-full text-xs sm:text-sm transition-all duration-300 border border-red-600 hover:border-red-500 font-bold tracking-wide shadow-lg shadow-red-500/30 hover:shadow-xl hover:shadow-red-500/40"
        >
          LOGOUT
        </button>
      </div>
    </div>
  </nav>

  <div class="flex pt-16">
    <!-- Sidebar - Fixed Position -->
    <aside class="fixed left-0 top-16 w-48 sm:w-64 bg-black border-r border-gray-800 h-screen overflow-y-auto z-40">
      <nav class="p-3 sm:p-4 space-y-1 sm:space-y-2">
        {#each navItems as item}
          <a
            href={item.href}
            class="w-full px-3 py-2 sm:px-4 sm:py-3 rounded-lg transition-all duration-300 flex items-center space-x-2 sm:space-x-3 group {isActive(item.href)
              ? 'bg-gray-700 text-white border border-gray-500 shadow-lg shadow-gray-500/10' 
              : 'text-gray-300 hover:text-white hover:bg-gray-700/20 border border-transparent hover:border-gray-500/30 hover:shadow-lg hover:shadow-gray-500/5 active:bg-gray-700/30 active:border-gray-500/40 active:shadow-xl active:shadow-gray-500/10'}"
          >
            <Icon name={item.icon} size="w-5 h-5 sm:w-6 sm:h-6" className="transition-transform duration-300 group-hover:scale-110" />
            <span class="font-medium text-sm sm:text-base">{item.label}</span>
            {#if isActive(item.href)}
              <div class="ml-auto w-1.5 h-1.5 sm:w-2 sm:h-2 bg-gray-400 rounded-full"></div>
            {/if}
          </a>
        {/each}
      </nav>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 bg-black min-h-screen overflow-x-auto ml-48 sm:ml-64">
      {#if isLoading}
        <div class="flex items-center justify-center h-64">
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"></div>
        </div>
      {:else}
        <slot />
      {/if}
    </main>
  </div>
</div>
