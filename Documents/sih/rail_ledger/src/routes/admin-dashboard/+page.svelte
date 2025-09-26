<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	onMount(() => {
		// Redirect to analytics page as the landing page
		goto('/admin-dashboard/analytics');
	});
</script>

<div class="flex min-h-screen items-center justify-center">
	<div class="text-center">
		<div
			class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-b-2 border-purple-500"
		></div>
		<p class="text-gray-400">Redirecting to Analytics...</p>
	</div>
</div>
