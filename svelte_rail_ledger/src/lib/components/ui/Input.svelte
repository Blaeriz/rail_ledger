<script>
  export let type = 'text';
  export let placeholder = '';
  export let value = '';
  export let disabled = false;
  export let required = false;
  export let label = '';
  export let error = '';
  
  $: inputId = `input-${Math.random().toString(36).substr(2, 9)}`;
</script>

<div class="input-group">
  {#if label}
    <label for={inputId} class="input-label">{label}</label>
  {/if}
  
  <input
    {type}
    {placeholder}
    {disabled}
    {required}
    id={inputId}
    bind:value
    class="input-field {error ? 'error' : ''}"
    on:input
    on:change
    on:keypress
    on:keydown
    on:keyup
    on:focus
    on:blur
  />
  
  {#if error}
    <span class="input-error">{error}</span>
  {/if}
</div>

<style>
  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .input-label {
    font-weight: 500;
    color: #374151;
    font-size: 0.875rem;
  }

  .input-field {
    padding: 0.75rem;
    border: 2px solid #e5e7eb;
    border-radius: 6px;
    font-size: 1rem;
    transition: border-color 0.3s;
    background: white;
  }

  .input-field:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .input-field:disabled {
    background-color: #f9fafb;
    cursor: not-allowed;
  }

  .input-field.error {
    border-color: #ef4444;
  }

  .input-error {
    color: #ef4444;
    font-size: 0.875rem;
  }
</style>
