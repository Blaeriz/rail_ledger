<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { isLoggedIn, getUserRole, getRedirectPath } from '$lib/utils/auth';
  import { ROUTES } from '$lib/utils/constants';

  onMount(() => {
    // Check if already logged in
    if (isLoggedIn()) {
      const role = getUserRole();
      goto(getRedirectPath(role));
    } else {
      goto(ROUTES.HOME);
    }
  });
</script>

<svelte:head>
  <title>Rail Ledger - Home</title>
</svelte:head>

<div class="loading-container">
  <div class="loading-spinner">
    <h1>🚆 Rail Ledger</h1>
    <p>Loading...</p>
  </div>
</div>

<style>
  .loading-container {
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  .loading-spinner {
    text-align: center;
    color: white;
  }

  .loading-spinner h1 {
    font-size: 2rem;
    margin: 0 0 1rem 0;
  }

  .loading-spinner p {
    font-size: 1.1rem;
    margin: 0;
  }
</style>