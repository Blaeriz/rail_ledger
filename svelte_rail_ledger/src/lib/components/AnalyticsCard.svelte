<script>
  export let title = '';
  export let value = 0;
  export let change = '';
  export let trend = 'neutral';
  export let color = '#3b82f6';
  export let icon = '';
</script>

<div class="analytics-card" style="border-left-color: {color}">
  <div class="card-header">
    <h3>{title}</h3>
    {#if change}
      <span class="change {trend}">{change}</span>
    {/if}
  </div>
  <div class="card-value">
    {#if icon}
      <span class="card-icon">{icon}</span>
    {/if}
    <span class="value">{value}</span>
  </div>
</div>

<style>
  .analytics-card {
    background: white;
    padding: 1rem 1.25rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #e5e7eb;
    transition: all 0.3s ease;
  }

  .analytics-card:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .card-header h3 {
    margin: 0;
    color: #64748b;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .change {
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
  }

  .change.up {
    background: #dcfce7;
    color: #166534;
  }

  .change.down {
    background: #fef2f2;
    color: #dc2626;
  }

  .change.neutral {
    background: #f3f4f6;
    color: #6b7280;
  }

  .card-value {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .card-icon {
    font-size: 1.25rem;
  }

  .value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #000000;
  }
</style>
