<script>
  export let variant = 'primary';
  export let size = 'medium';
  export let disabled = false;
  export let type = 'button';
  
  $: classes = `btn btn-${variant} btn-${size} ${disabled ? 'disabled' : ''}`;
</script>

<button {type} {disabled} class={classes} on:click>
  <slot />
</button>

<style>
  .btn {
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.3s;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .btn:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  /* Variants */
  .btn-primary {
    background: #3b82f6;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
  }

  .btn-secondary {
    background: #6b7280;
    color: white;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #4b5563;
  }

  .btn-success {
    background: #10b981;
    color: white;
  }

  .btn-success:hover:not(:disabled) {
    background: #059669;
  }

  .btn-danger {
    background: #ef4444;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #dc2626;
  }

  /* Sizes */
  .btn-small {
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
  }

  .btn-medium {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
  }

  .btn-large {
    padding: 1rem 2rem;
    font-size: 1.125rem;
  }
</style>
