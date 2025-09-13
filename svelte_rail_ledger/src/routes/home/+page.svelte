<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => {
    // Check if already logged in
    const loginStatus = localStorage.getItem('isLoggedIn');
    if (loginStatus === 'true') {
      const role = localStorage.getItem('role');
      if (role === 'Admin') {
        goto('/admin/dashboard');
      } else if (role === 'Inspector') {
        goto('/inspector/dashboard');
      } else if (role === 'Viewer') {
        goto('/viewer/dashboard');
      }
    }
  });

  function goToLogin() {
    goto('/login');
  }
</script>

<svelte:head>
  <title>Rail Ledger - Home</title>
</svelte:head>

<div class="landing-container">
  <div class="hero-section">
    <div class="hero-content">
      <div class="logo-section">
        <span class="logo-icon">🚆</span>
        <h1 class="logo-text">Rail Ledger</h1>
      </div>
      <h2 class="hero-title">Digital Track Fittings Management for Indian Railways</h2>
      <p class="hero-description">
        Streamline your railway operations with our comprehensive digital platform for managing vendors, batches, inspections, and fitments.
      </p>
      <button class="cta-button" on:click={goToLogin}>
        Get Started
      </button>
    </div>
  </div>

  <div class="features-section">
    <div class="features-grid">
      <div class="feature-card">
        <div class="feature-icon">🏢</div>
        <h3>Vendor Management</h3>
        <p>Comprehensive vendor tracking with GST, PAN, and audit information</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">📦</div>
        <h3>Batch Tracking</h3>
        <p>Real-time batch monitoring with QR codes and quality control status</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">🔍</div>
        <h3>Inspections</h3>
        <p>Digital inspection workflows with mobile QR scanning capabilities</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">📊</div>
        <h3>Analytics</h3>
        <p>AI-powered insights and reports for better decision making</p>
      </div>
    </div>
  </div>

  <div class="footer">
    <p>&copy; 2024 Rail Ledger. Railway Management System.</p>
  </div>
</div>

<style>
  .landing-container {
    min-height: 100vh;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    color: white;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  .hero-section {
    padding: 4rem 2rem;
    text-align: center;
    min-height: 80vh;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .hero-content {
    max-width: 800px;
  }

  .logo-section {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .logo-icon {
    font-size: 4rem;
  }

  .logo-text {
    font-size: 3rem;
    font-weight: bold;
    margin: 0;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .hero-title {
    font-size: 2.5rem;
    margin: 0 0 1.5rem 0;
    font-weight: 600;
    line-height: 1.2;
  }

  .hero-description {
    font-size: 1.2rem;
    margin: 0 0 2.5rem 0;
    opacity: 0.9;
    line-height: 1.6;
  }

  .cta-button {
    background: white;
    color: #000000;
    border: none;
    padding: 1rem 2rem;
    border-radius: 8px;
    font-size: 1.1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .cta-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
  }

  @media (max-width: 768px) {
    .home-page {
      padding: 2rem 1rem;
    }
    
    .hero-section {
      padding: 2rem 0;
    }
    
    .logo-text {
      font-size: 2.5rem;
    }
    
    .hero-title {
      font-size: 2rem;
    }
    
    .hero-description {
      font-size: 1.1rem;
    }
    
    .cta-button {
      padding: 1rem 2rem;
      font-size: 1.1rem;
    }
  }

  @media (max-width: 480px) {
    .home-page {
      padding: 1rem 0.5rem;
    }
    
    .logo-text {
      font-size: 2rem;
    }
    
    .hero-title {
      font-size: 1.75rem;
    }
    
    .hero-description {
      font-size: 1rem;
    }
    
    .cta-button {
      padding: 0.875rem 1.5rem;
      font-size: 1rem;
    }
  }

  .features-section {
    background: white;
    color: #000000;
    padding: 4rem 2rem;
  }

  .features-grid {
    max-width: 1200px;
    margin: 0 auto;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
  }

  .feature-card {
    text-align: center;
    padding: 2rem;
    border-radius: 12px;
    background: #f8fafc;
    transition: transform 0.3s;
  }

  .feature-card:hover {
    transform: translateY(-4px);
  }

  .feature-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .feature-card h3 {
    font-size: 1.5rem;
    margin: 0 0 1rem 0;
    color: #000000;
  }

  .feature-card p {
    color: #64748b;
    line-height: 1.6;
    margin: 0;
  }

  .footer {
    background: #1e293b;
    color: white;
    text-align: center;
    padding: 2rem;
  }

  .footer p {
    margin: 0;
    opacity: 0.8;
  }

  @media (max-width: 768px) {
    .hero-title {
      font-size: 2rem;
    }
    
    .logo-text {
      font-size: 2.5rem;
    }
    
    .features-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
