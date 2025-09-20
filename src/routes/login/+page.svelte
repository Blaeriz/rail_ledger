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
      const user = (userKey === 'admin' || userKey === 'inspector' || userKey === 'viewer') 
        ? credentials[userKey] 
        : null;
      
      if (user && user.password === password) {
        // Store user session
        sessionStorage.setItem('user', JSON.stringify({
          username: username.toLowerCase(),
          role: user.role,
          loginTime: new Date().toISOString()
        }));
        
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

<div class="min-h-screen bg-gradient-to-br from-gray-900 via-black to-gray-900 flex items-center justify-center px-4">
  <!-- Background Pattern -->
  <div class="absolute inset-0 bg-pattern opacity-5"></div>
  
  <!-- Login Card -->
  <div class="relative w-full max-w-md">
    <!-- Glass morphism effect -->
    <div class="backdrop-blur-xl bg-gray-900/40 border border-gray-700/40 rounded-3xl p-8 shadow-2xl">
      <!-- Header -->
      <div class="text-center mb-8">
        <h1 class="text-3xl font-bold text-white mb-2">Rail Ledger</h1>
        <p class="text-gray-400">Sign in to your account</p>
      </div>

      <!-- Login Form -->
      <form on:submit|preventDefault={handleLogin} class="space-y-6">
        <!-- Username Field -->
        <div>
          <label for="username" class="block text-sm font-medium text-gray-300 mb-2">
            Username
          </label>
          <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
              </svg>
            </div>
            <input
              id="username"
              type="text"
              bind:value={username}
              on:keypress={handleKeyPress}
              class="w-full pl-10 pr-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all duration-300"
              placeholder="Enter username"
              required
            />
          </div>
        </div>

        <!-- Password Field -->
        <div>
          <label for="password" class="block text-sm font-medium text-gray-300 mb-2">
            Password
          </label>
          <div class="relative">
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
              </svg>
            </div>
            <input
              id="password"
              type="password"
              bind:value={password}
              on:keypress={handleKeyPress}
              class="w-full pl-10 pr-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all duration-300"
              placeholder="Enter password"
              required
            />
          </div>
        </div>

        <!-- Error Message -->
        {#if error}
          <div class="bg-red-900/20 border border-red-500/30 text-red-300 px-4 py-3 rounded-xl text-sm">
            {error}
          </div>
        {/if}

        <!-- Login Button -->
        <button
          type="submit"
          disabled={isLoading}
          class="w-full bg-gray-800 hover:bg-gray-700 disabled:bg-gray-600/60 text-white font-bold py-3 px-6 rounded-lg transition-all duration-300 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30 disabled:scale-100 disabled:cursor-not-allowed"
        >
          {#if isLoading}
            <div class="flex items-center justify-center">
              <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Signing in...
            </div>
          {:else}
            Sign In
          {/if}
        </button>
      </form>

      <!-- Demo Credentials -->
      <div class="mt-8 p-4 bg-gray-800/30 rounded-xl border border-gray-700/30">
        <h3 class="text-sm font-medium text-gray-300 mb-3">Demo Credentials:</h3>
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
      <div class="text-center mt-6">
        <a href="/" class="text-gray-400 hover:text-white text-sm transition-colors duration-300">
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
