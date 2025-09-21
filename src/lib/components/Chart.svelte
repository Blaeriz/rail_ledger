<!-- @ts-nocheck -->
<script>
	import { onMount, onDestroy } from 'svelte';
	import { Chart, registerables } from 'chart.js';

	Chart.register(...registerables);

	export let type = 'doughnut';
	export let data = {};
	export let options = {};

	let canvas = /** @type {HTMLCanvasElement | null} */ (null);
	let chartInstance = /** @type {any} */ (null);

	onMount(() => {
		if (canvas) {
			const config = {
				type: type,
				data: data,
				options: {
					responsive: true,
					maintainAspectRatio: false,
					...options
				}
			};
			// @ts-expect-error Chart.js type compatibility
			chartInstance = new Chart(canvas, config);
		}
	});

	onDestroy(() => {
		if (chartInstance) {
			chartInstance.destroy();
		}
	});

	// Update chart when data changes
	$: if (chartInstance && data) {
		chartInstance.data = data;
		chartInstance.update();
	}
</script>

<div class="relative h-full w-full">
	<canvas bind:this={canvas}></canvas>
</div>
