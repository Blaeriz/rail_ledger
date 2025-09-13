<script>
  import { goto } from '$app/navigation';
  
  export let userRole = '';
  export let activePage = '';
  export let sidebarOpen = true;

  const adminLinks = [
    { 
      name: 'Dashboard', 
      path: '/admin/dashboard', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 13H11V3H3V13ZM3 21H11V15H3V21ZM13 21H21V11H13V21ZM13 3V9H21V3H13Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    },
    { 
      name: 'Vendors', 
      path: '/admin/vendors', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 21H21M5 21V7L12 3L19 7V21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M9 9H15M9 13H15M9 17H13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>`
    },
    { 
      name: 'Batches', 
      path: '/admin/batches', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M20 7L12 3L4 7M20 7V17L12 21M20 7L12 11M4 7V17L12 21M4 7L12 11" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="12" cy="12" r="2" stroke="currentColor" stroke-width="1.5"/>
      </svg>`
    },
    { 
      name: 'Users', 
      path: '/admin/users', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M20 21V19C20 17.9391 19.5786 16.9217 18.8284 16.1716C18.0783 15.4214 17.0609 15 16 15H8C6.93913 15 5.92172 15.4214 5.17157 16.1716C4.42143 16.9217 4 17.9391 4 19V21M16 7C16 9.20914 14.2091 11 12 11C9.79086 11 8 9.20914 8 7C8 4.79086 9.79086 3 12 3C14.2091 3 16 4.79086 16 7Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    },
    { 
      name: 'Reports', 
      path: '/reports', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M9 17H15M9 13H15M9 9H12M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    }
  ];

  const inspectorLinks = [
    { 
      name: 'Dashboard', 
      path: '/inspector/dashboard', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 13H11V3H3V13ZM3 21H11V15H3V21ZM13 21H21V11H13V21ZM13 3V9H21V3H13Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    },
    { 
      name: 'Scan QR', 
      path: '/inspector/scan', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 7V5C3 3.89543 3.89543 3 5 3H7M17 3H19C20.1046 3 21 3.89543 21 5V7M21 17V19C21 20.1046 20.1046 21 19 21H17M7 21H5C3.89543 21 3 20.1046 3 19V17M8 12H16M12 8V16" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <rect x="9" y="9" width="6" height="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    },
    { 
      name: 'Inspections', 
      path: '/inspector/inspections', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M9 12L11 14L15 10M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M8 7H16M8 11H12M8 15H14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>`
    },
    { 
      name: 'Fitments', 
      path: '/inspector/fitments', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M14.7 6.3C15.1 5.9 15.1 5.3 14.7 4.9L13.8 4C13.4 3.6 12.8 3.6 12.4 4L11.5 4.9C11.1 5.3 11.1 5.9 11.5 6.3L12.4 7.2C12.8 7.6 13.4 7.6 13.8 7.2L14.7 6.3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M9.3 11.3C9.7 10.9 9.7 10.3 9.3 9.9L8.4 9C8 8.6 7.4 8.6 7 9L6.1 9.9C5.7 10.3 5.7 10.9 6.1 11.3L7 12.2C7.4 12.6 8 12.6 8.4 12.2L9.3 11.3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M19.3 11.3C19.7 10.9 19.7 10.3 19.3 9.9L18.4 9C18 8.6 17.4 8.6 17 9L16.1 9.9C15.7 10.3 15.7 10.9 16.1 11.3L17 12.2C17.4 12.6 18 12.6 18.4 12.2L19.3 11.3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M9.3 16.3C9.7 15.9 9.7 15.3 9.3 14.9L8.4 14C8 13.6 7.4 13.6 7 14L6.1 14.9C5.7 15.3 5.7 15.9 6.1 16.3L7 17.2C7.4 17.6 8 17.6 8.4 17.2L9.3 16.3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M19.3 16.3C19.7 15.9 19.7 15.3 19.3 14.9L18.4 14C18 13.6 17.4 13.6 17 14L16.1 14.9C15.7 15.3 15.7 15.9 16.1 16.3L17 17.2C17.4 17.6 18 17.6 18.4 17.2L19.3 16.3Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    }
  ];

  const viewerLinks = [
    { 
      name: 'Dashboard', 
      path: '/viewer/dashboard', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 13H11V3H3V13ZM3 21H11V15H3V21ZM13 21H21V11H13V21ZM13 3V9H21V3H13Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    },
    { 
      name: 'Analytics', 
      path: '/viewer/analytics', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M3 3V21H21M8 16L12 12L16 8L20 4" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="8" cy="16" r="2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="12" cy="12" r="2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="16" cy="8" r="2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="20" cy="4" r="2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    },
    { 
      name: 'Reports', 
      path: '/reports', 
      icon: `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M9 17H15M9 13H15M9 9H12M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>`
    }
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
          <span class="nav-icon">{@html link.icon}</span>
          <span class="nav-text">{link.name}</span>
        </button>
      {/each}
    </nav>
  </div>
</aside>

<style>
  .sidebar {
    width: 280px;
    background: linear-gradient(180deg, #ffffff 0%, #f8fafc 100%);
    border-right: 1px solid #e2e8f0;
    height: calc(100vh - 60px);
    position: fixed;
    left: 0;
    top: 60px;
    overflow-y: auto;
    z-index: 999;
    box-shadow: 4px 0 20px rgba(0, 0, 0, 0.08);
    transition: transform 0.3s ease;
  }

  .sidebar.open {
    transform: translateX(0);
  }

  .sidebar.closed {
    transform: translateX(-100%);
  }

  .sidebar-content {
    padding: 2rem 0;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0 1.5rem;
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem 1.25rem;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    transition: all 0.3s ease;
    color: #64748b;
    font-size: 0.95rem;
    width: 100%;
    border-radius: 12px;
    position: relative;
    font-weight: 500;
    margin-bottom: 0.25rem;
  }

  .nav-link:hover {
    background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);
    color: #1e293b;
    transform: translateX(6px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }

  .nav-link.active {
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    color: white;
    font-weight: 600;
    box-shadow: 0 8px 24px rgba(30, 58, 138, 0.3);
    transform: translateX(6px);
  }

  .nav-link.active::before {
    content: '';
    position: absolute;
    left: -1.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 4px;
    height: 24px;
    background: linear-gradient(135deg, #3b82f6 0%, #1e40af 100%);
    border-radius: 2px;
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.4);
  }

  .nav-icon {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }


  .nav-text {
    font-weight: 500;
    letter-spacing: 0.025em;
  }

  @media (max-width: 768px) {
    .sidebar {
      width: 300px;
      height: 100vh;
      top: 0;
      z-index: 1001;
    }

    .sidebar.closed {
      transform: translateX(-100%);
    }
    
    .sidebar-content {
      padding: 1.5rem 0;
    }
    
    .sidebar-nav {
      padding: 0 1rem;
    }
    
    .nav-link {
      padding: 1.25rem 1rem;
      font-size: 1rem;
      gap: 1.25rem;
    }

    .nav-link:hover,
    .nav-link.active {
      transform: translateX(4px);
    }
  }

  @media (max-width: 480px) {
    .sidebar {
      width: 100%;
    }
    
    .nav-link {
      padding: 1.5rem 1.25rem;
      gap: 1.5rem;
    }

    .nav-icon {
      width: 24px;
      height: 24px;
    }

  }
</style>
