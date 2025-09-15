<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { isLoggedIn, getUserRole, login, getRedirectPath } from '$lib/utils/auth';
  import { DEMO_CREDENTIALS } from '$lib/utils/constants';

  let username = '';
  let password = '';
  let error = '';
  let isLoading = false;

  onMount(() => {
    // Check if already logged in
    if (isLoggedIn()) {
      const role = getUserRole();
      goto(getRedirectPath(role));
    }
  });

  async function handleLogin() {
    if (!username || !password) {
      error = 'Please enter both username and password';
      return;
    }

    isLoading = true;
    error = '';

    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 1000));

    const user = DEMO_CREDENTIALS.find(
      (u) => u.username === username && u.password === password
    );

    if (user) {
      login(username, user.role);
      goto(getRedirectPath(user.role));
    } else {
      error = 'Invalid username or password';
    }

    isLoading = false;
  }

  function handleKeyPress(event) {
    if (event.key === 'Enter') {
      handleLogin();
    }
  }
</script>

<svelte:head>
  <title>Rail Ledger - Login</title>
</svelte:head>

<div class="login-container">
  <div class="login-card">
    <div class="header">
      <div class="logo">
        <span class="logo-icon">🚆</span>
        <h1>Rail_Ledger</h1>
      </div>
      <p class="subtitle">Digital Track Fittings Management for Indian Railways</p>
    </div>

    <form on:submit|preventDefault={handleLogin}>
      <div class="input-group">
        <label for="username">Username</label>
        <input
          id="username"
          type="text"
          placeholder="Enter your username"
          bind:value={username}
          on:keypress={handleKeyPress}
          disabled={isLoading}
        />
      </div>

      <div class="input-group">
        <label for="password">Password</label>
        <input
          id="password"
          type="password"
          placeholder="Enter your password"
          bind:value={password}
          on:keypress={handleKeyPress}
          disabled={isLoading}
        />
      </div>

      {#if error}
        <div class="error-message">{error}</div>
      {/if}

      <button type="submit" disabled={isLoading} class="login-btn">
        {#if isLoading}
          Logging in...
        {:else}
          Login
        {/if}
      </button>
    </form>

    <div class="demo-credentials">
      <h3>Demo Credentials</h3>
      <div class="credential-item">
        <strong>Admin:</strong> admin / admin123
      </div>
      <div class="credential-item">
        <strong>Inspector:</strong> inspector / insp123
      </div>
      <div class="credential-item">
        <strong>Viewer:</strong> viewer / view123
      </div>
    </div>
  </div>
</div>

<style>
  .login-container {
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    padding: 1rem;
  }

  .login-card {
    background: white;
    padding: 2.5rem;
    border-radius: 12px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 450px;
    text-align: center;
  }

  .header {
    margin-bottom: 2rem;
  }

  .logo {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .logo-icon {
    font-size: 2.5rem;
  }

  .logo h1 {
    color: #1e3a8a;
    margin: 0;
    font-size: 2rem;
    font-weight: bold;
  }

  .subtitle {
    color: #64748b;
    font-size: 0.9rem;
    margin: 0;
  }

  .input-group {
    margin-bottom: 1.5rem;
    text-align: left;
  }

  .input-group label {
    display: block;
    margin-bottom: 0.5rem;
    color: #374151;
    font-weight: 500;
  }

  input {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.3s;
    box-sizing: border-box;
  }

  input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  input:disabled {
    background-color: #f9fafb;
    cursor: not-allowed;
  }

  .login-btn {
    width: 100%;
    padding: 0.875rem;
    background: #1e3a8a;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.3s;
    margin-top: 1rem;
  }

  .login-btn:hover:not(:disabled) {
    background: #1e40af;
  }

  .login-btn:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }

  .error-message {
    color: #ef4444;
    font-size: 0.9rem;
    margin-top: 0.5rem;
    padding: 0.5rem;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 4px;
  }

  .demo-credentials {
    margin-top: 2rem;
    padding: 1.5rem;
    background: #f8fafc;
    border-radius: 8px;
    text-align: left;
  }

  .demo-credentials h3 {
    margin: 0 0 1rem 0;
    color: #374151;
    font-size: 1rem;
  }

  .credential-item {
    margin: 0.5rem 0;
    font-size: 0.9rem;
    color: #6b7280;
  }

  @media (max-width: 768px) {
    .login-container {
      padding: 0.5rem;
    }
    
    .login-card {
      padding: 2rem;
    }
    
    .logo-text {
      font-size: 1.75rem;
    }
    
    .header h1 {
      font-size: 1.5rem;
    }
  }

  @media (max-width: 480px) {
    .login-card {
      padding: 1.5rem;
    }
    
    .logo-text {
      font-size: 1.5rem;
    }
    
    .header h1 {
      font-size: 1.25rem;
    }
    
    .input-group input {
      padding: 0.75rem;
    }
    
    .login-btn {
      padding: 0.75rem;
    }
  }
</style>
