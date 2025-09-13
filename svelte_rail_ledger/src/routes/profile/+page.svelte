<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/profile';
  let userDetails = {};

  onMount(() => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
      userDetails = {
        aadhar: '123456789012',
        phone: '+91-9876543210',
        name: username,
        role: userRole
      };
    }
  });
</script>

<Layout {userRole} {username} {activePage}>
  <div class="profile-page">
    <h1>Profile & Settings</h1>
    
    <div class="profile-card">
      <h2>User Details</h2>
      <div class="details-grid">
        <div class="detail-item">
          <label>Aadhaar:</label>
          <span>{userDetails.aadhar}</span>
        </div>
        <div class="detail-item">
          <label>Phone:</label>
          <span>{userDetails.phone}</span>
        </div>
        <div class="detail-item">
          <label>Name:</label>
          <span>{userDetails.name}</span>
        </div>
        <div class="detail-item">
          <label>Role:</label>
          <span class="role role-{userDetails.role?.toLowerCase()}">{userDetails.role}</span>
        </div>
      </div>
    </div>
  </div>
</Layout>

<style>
  .profile-page {
    padding: 2rem;
  }

  .profile-page h1 {
    color: #000000;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .profile-card {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .profile-card h2 {
    color: #000000;
    margin: 0 0 1.5rem 0;
    font-size: 1.5rem;
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .detail-item label {
    font-weight: 600;
    color: #374151;
  }

  .role {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
    display: inline-block;
    width: fit-content;
  }

  .role-admin {
    background: #dbeafe;
    color: #1e40af;
  }

  .role-inspector {
    background: #f3e8ff;
    color: #7c3aed;
  }

  .role-viewer {
    background: #dcfce7;
    color: #166534;
  }
</style>
