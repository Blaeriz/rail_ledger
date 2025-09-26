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
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4 backdrop-blur-sm"
		on:click={handleBackdropClick}
		on:keydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div
			class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-lg border border-gray-600 bg-gray-800 p-4 sm:p-6"
		>
			<div class="mb-6 flex items-center justify-between">
				<h3 class="text-xl font-bold text-white sm:text-2xl">{title}</h3>
				<button
					on:click={onClose}
					class="rounded border border-transparent p-1 text-2xl text-gray-400 transition-colors duration-300 hover:border-gray-500/30 hover:bg-gray-700/20 hover:text-white"
					aria-label="Close modal"
				>
					×
				</button>
			</div>

			<slot />
		</div>
	</div>
{/if}
