<script>
  import { goto } from '$app/navigation';
  import { createEventDispatcher } from 'svelte';
  
  export let userRole = '';
  export let username = '';

  const dispatch = createEventDispatcher();

  function handleLogout() {
    localStorage.removeItem('isLoggedIn');
    localStorage.removeItem('username');
    localStorage.removeItem('role');
    goto('/login');
  }

  function goToProfile() {
    goto('/profile');
  }

  function toggleSidebar() {
    dispatch('toggleSidebar');
  }

  function getDashboardPath() {
    switch (userRole) {
      case 'Admin':
        return '/admin/dashboard';
      case 'Inspector':
        return '/inspector/dashboard';
      case 'Viewer':
        return '/viewer/dashboard';
      default:
        return '/home';
    }
  }
</script>

<nav class="navbar">
  <div class="navbar-brand">
    <button class="menu-toggle" on:click={toggleSidebar} aria-label="Toggle sidebar menu">
      <span class="hamburger"></span>
      <span class="hamburger"></span>
      <span class="hamburger"></span>
    </button>
    <a href={getDashboardPath()} class="logo">
      <span class="logo-icon">🚆</span>
      <span class="logo-text">Rail_Ledger</span>
    </a>
  </div>
  
  <div class="navbar-actions">
    <span class="user-info">
      {username} ({userRole})
    </span>
    <button class="profile-btn" on:click={goToProfile}>
      Profile
    </button>
    <button class="logout-btn" on:click={handleLogout}>
      Logout
    </button>
  </div>
</nav>

<style>
  .navbar {
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    color: white;
    padding: 0.5rem 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(255, 255, 255, 0.05);
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
    height: 60px;
    backdrop-filter: blur(20px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .navbar::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(90deg, transparent 0%, rgba(59, 130, 246, 0.1) 50%, transparent 100%);
    pointer-events: none;
  }

  .navbar-brand {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .menu-toggle {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    cursor: pointer;
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    border-radius: 8px;
    transition: all 0.3s ease;
  }

  .menu-toggle:hover {
    background: rgba(255, 255, 255, 0.2);
    transform: scale(1.05);
  }

  .hamburger {
    width: 20px;
    height: 2px;
    background: white;
    transition: all 0.3s;
    border-radius: 1px;
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 1.4rem;
    font-weight: 700;
    color: white;
    text-decoration: none;
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .logo:hover {
    transform: scale(1.02);
    color: white;
    text-decoration: none;
  }

  .logo-icon {
    font-size: 1.8rem;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
  }

  .logo-text {
    font-size: 1.25rem;
    font-weight: bold;
  }

  .navbar-actions {
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .user-info {
    font-size: 0.9rem;
    color: #e2e8f0;
    font-weight: 500;
    padding: 0.5rem 1rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .profile-btn, .logout-btn {
    background: rgba(255, 255, 255, 0.08);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.15);
    padding: 0.6rem 1.2rem;
    border-radius: 10px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 600;
    transition: all 0.3s ease;
    backdrop-filter: blur(20px);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    position: relative;
    overflow: hidden;
  }

  .profile-btn::before, .logout-btn::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    transition: left 0.5s;
  }

  .profile-btn:hover::before, .logout-btn:hover::before {
    left: 100%;
  }

  .profile-btn:hover {
    background: rgba(59, 130, 246, 0.9);
    border-color: #3b82f6;
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(59, 130, 246, 0.4);
  }

  .logout-btn {
    background: #ef4444;
    border-color: #ef4444;
    color: white;
  }

  .logout-btn:hover {
    background: #dc2626;
    border-color: #dc2626;
    color: white;
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(239, 68, 68, 0.4);
  }

  @media (max-width: 768px) {
    .navbar {
      padding: 0.5rem 1rem;
    }
    
    .navbar-brand {
      gap: 0.5rem;
    }
    
    .logo {
      font-size: 1.2rem;
    }
    
    .logo-icon {
      font-size: 1.5rem;
    }
    
    .navbar-actions {
      gap: 0.5rem;
    }
    
    .user-info {
      display: none;
    }
    
    .profile-btn, .logout-btn {
      padding: 0.4rem 0.8rem;
      font-size: 0.8rem;
    }
  }

  @media (min-width: 769px) {
    .menu-toggle {
      display: none;
    }
  }
</style>
