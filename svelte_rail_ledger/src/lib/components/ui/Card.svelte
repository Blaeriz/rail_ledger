<script>
  export let title = '';
  export let padding = 'medium';
  
  $: paddingClass = `card-padding-${padding}`;
</script>

<div class="card {paddingClass}">
  {#if title}
    <h3 class="card-title">{title}</h3>
  {/if}
  <div class="card-content">
    <slot />
  </div>
</div>

<style>
  .card {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border: 1px solid #e5e7eb;
  }

  .card-padding-small {
    padding: 1rem;
  }

  .card-padding-medium {
    padding: 1.5rem;
  }

  .card-padding-large {
    padding: 2rem;
  }

  .card-title {
    margin: 0 0 1rem 0;
    color: #000000;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .card-content {
    color: #64748b;
  }
</style>
