<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  onMount(() => {
    // Redirect to quality analysis page as the landing page
    goto('/inspector-dashboard/quality');
  });
</script>

<div class="flex items-center justify-center min-h-screen">
  <div class="text-center">
    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500 mx-auto mb-4"></div>
    <p class="text-gray-400">Redirecting to Quality Analysis...</p>
  </div>
</div>