<!-- @ts-nocheck -->
<script>
  export let isOpen = false;
  export let title = '';
  export let onClose = () => {};
  
  function handleBackdropClick(/** @type {any} */ event) {
    if (event.target === event.currentTarget) {
      onClose();
    }
  }
  
  function handleKeydown(/** @type {any} */ event) {
    if (event.key === 'Escape') {
      onClose();
    }
  }
</script>

{#if isOpen}
  <div 
    class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 p-4"
    on:click={handleBackdropClick}
    on:keydown={handleKeydown}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="bg-gray-800 border border-gray-600 rounded-lg p-4 sm:p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-xl sm:text-2xl font-bold text-white">{title}</h3>
        <button 
          on:click={onClose}
          class="text-gray-400 hover:text-white text-2xl transition-colors duration-300 hover:bg-gray-700/20 p-1 rounded border border-transparent hover:border-gray-500/30"
          aria-label="Close modal"
        >
          ×
        </button>
      </div>
      
      <slot />
    </div>
  </div>
{/if}
