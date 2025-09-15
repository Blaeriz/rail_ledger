<script>
  import { goto } from '$app/navigation';
  
  export let userRole = '';
  export let activePage = '';
  export let sidebarOpen = true;

  const adminLinks = [
    { name: 'Dashboard', path: '/admin/dashboard', icon: '📊' },
    { name: 'Vendors', path: '/admin/vendors', icon: '🏢' },
    { name: 'Batches', path: '/admin/batches', icon: '📦' },
    { name: 'Users', path: '/admin/users', icon: '👥' },
    { name: 'Reports', path: '/reports', icon: '📈' }
  ];

  const inspectorLinks = [
    { name: 'Dashboard', path: '/inspector/dashboard', icon: '📊' },
    { name: 'Scan QR', path: '/inspector/scan', icon: '📱' },
    { name: 'Inspections', path: '/inspector/inspections', icon: '🔍' },
    { name: 'Fitments', path: '/inspector/fitments', icon: '🔧' }
  ];

  const viewerLinks = [
    { name: 'Dashboard', path: '/viewer/dashboard', icon: '📊' },
    { name: 'Analytics', path: '/viewer/analytics', icon: '📈' },
    { name: 'Reports', path: '/reports', icon: '📋' }
  ];

  let links = [];

  $: {
    if (userRole === 'Admin') {
      links = adminLinks;
    } else if (userRole === 'Inspector') {
      links = inspectorLinks;
    } else if (userRole === 'Viewer') {
      links = viewerLinks;
    }
  }

  function navigateTo(path) {
    goto(path);
  }
</script>

<aside class="sidebar {sidebarOpen ? 'open' : 'closed'}">
  <div class="sidebar-content">
    <nav class="sidebar-nav">
      {#each links as link}
        <button
          class="nav-link {activePage === link.path ? 'active' : ''}"
          on:click={() => navigateTo(link.path)}
        >
          <span class="nav-icon">{link.icon}</span>
          <span class="nav-text">{link.name}</span>
        </button>
      {/each}
    </nav>
  </div>
</aside>

<style>
  .sidebar {
    width: 250px;
    background: white;
    border-right: 1px solid #e2e8f0;
    height: calc(100vh - 60px);
    position: fixed;
    left: 0;
    top: 60px;
    overflow-y: auto;
    z-index: 999;
    box-shadow: 2px 0 8px rgba(0, 0, 0, 0.08);
    transition: transform 0.3s ease;
  }

  .sidebar.open {
    transform: translateX(0);
  }

  .sidebar.closed {
    transform: translateX(-100%);
  }

  .sidebar-content {
    padding: 1.5rem 0;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0 1rem;
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.875rem 1rem;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    transition: all 0.3s ease;
    color: #64748b;
    font-size: 0.9rem;
    width: 100%;
    border-radius: 8px;
    position: relative;
    font-weight: 500;
  }

  .nav-link:hover {
    background: #f1f5f9;
    color: #1e293b;
    transform: translateX(4px);
  }

  .nav-link.active {
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    color: white;
    font-weight: 600;
    box-shadow: 0 4px 12px rgba(30, 58, 138, 0.3);
  }

  .nav-link.active::before {
    content: '';
    position: absolute;
    left: -1rem;
    top: 50%;
    transform: translateY(-50%);
    width: 4px;
    height: 20px;
    background: #3b82f6;
    border-radius: 2px;
  }

  .nav-icon {
    font-size: 1.1rem;
  }

  .nav-text {
    font-weight: 500;
  }

  @media (max-width: 768px) {
    .sidebar {
      width: 280px;
      height: 100vh;
      top: 0;
      z-index: 1001;
    }

    .sidebar.closed {
      transform: translateX(-100%);
    }
    
    .sidebar-content {
      padding: 1rem 0;
    }
    
    .sidebar-nav {
      padding: 0 0.5rem;
    }
    
    .nav-link {
      padding: 1rem 0.75rem;
      font-size: 1rem;
    }
  }

  @media (max-width: 480px) {
    .sidebar {
      width: 100%;
    }
    
    .nav-link {
      padding: 1.25rem 1rem;
    }
  }
</style>
