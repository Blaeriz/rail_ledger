<script>
  import Navbar from './Navbar.svelte';
  import Sidebar from './Sidebar.svelte';
  import { onMount } from 'svelte';
  import { isLoggedIn as checkLogin, getUserRole, getUsername } from '$lib/utils/auth';
  
  export let userRole = '';
  export let username = '';
  export let activePage = '';

  let isLoggedIn = false;
  let sidebarOpen = true;

  onMount(() => {
    isLoggedIn = checkLogin();
    if (isLoggedIn) {
      userRole = getUserRole();
      username = getUsername();
    }
  });

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }
</script>

{#if isLoggedIn}
  <div class="layout">
    <Navbar {userRole} {username} on:toggleSidebar={toggleSidebar} />
    <div class="main-content">
      <Sidebar {userRole} {activePage} {sidebarOpen} />
      <main class="content {sidebarOpen ? 'sidebar-open' : 'sidebar-closed'}">
        <slot />
      </main>
    </div>
  </div>
{:else}
  <slot />
{/if}

<style>
  .layout {
    min-height: 100vh;
    background: #f8fafc;
    display: flex;
    flex-direction: column;
  }

  .main-content {
    display: flex;
    flex: 1;
    margin-top: 60px;
  }

  .content {
    flex: 1;
    padding: 2rem;
    min-height: calc(100vh - 60px);
    background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
    transition: margin-left 0.3s ease;
  }

  .content.sidebar-open {
    margin-left: 250px;
  }

  .content.sidebar-closed {
    margin-left: 0;
  }

  @media (max-width: 768px) {
    .main-content {
      flex-direction: column;
    }
    
    .content {
      margin-left: 0 !important;
      padding: 1rem;
    }
    
    .content.sidebar-open {
      margin-left: 0;
    }
  }

  @media (max-width: 480px) {
    .content {
      padding: 0.75rem;
    }
  }
</style>
