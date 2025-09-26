<!-- @ts-nocheck -->
<script>
	import { onMount, createEventDispatcher } from 'svelte';
	import { scaleQuantile } from 'd3-scale';
	import * as d3 from 'd3';
	import * as topojson from 'topojson-client';

	export let data = [];
	export let width = 600;
	export let height = 600;
	export let projectionConfig = {
		scale: 800,
		center: [78.9629, 22.5937] // Center of India
	};

	const dispatch = createEventDispatcher();
	let mapContainer;
	let topoData = null;
	let isLoading = true;

	// Color scale for heatmap
	const colorScale = scaleQuantile()
		.domain([0, 10, 20, 30, 40, 50])
		.range(['#6b7280', '#22c55e', '#eab308', '#f97316', '#ef4444']);

	onMount(async () => {
		try {
			// Load TopoJSON data
			const response = await fetch('/india.topo.json');
			if (response.ok) {
				topoData = await response.json();
				console.log('TopoJSON loaded:', topoData);
				renderMap();
			} else {
				console.error('Failed to load map data:', response.status, response.statusText);
				// Create a fallback simple map
				createFallbackMap();
			}
		} catch (err) {
			console.error('Error loading map data:', err);
			// Create a fallback simple map
			createFallbackMap();
		} finally {
			isLoading = false;
		}
	});

	function createFallbackMap() {
		if (!mapContainer) return;
		
		// Clear previous content
		d3.select(mapContainer).selectAll('*').remove();
		
		// Create a simple SVG with text indicating no map data
		const svg = d3.select(mapContainer)
			.append('svg')
			.attr('width', '100%')
			.attr('height', '100%')
			.attr('viewBox', `0 0 ${width} ${height}`)
			.attr('preserveAspectRatio', 'xMidYMid meet')
			.style('background', 'transparent');
		
		svg.append('text')
			.attr('x', width / 2)
			.attr('y', height / 2)
			.attr('text-anchor', 'middle')
			.attr('fill', '#6b7280')
			.attr('font-size', '16px')
			.text('Map data not available');
	}

	function renderMap() {
		if (!mapContainer || !topoData) return;

		// Clear previous content
		d3.select(mapContainer).selectAll('*').remove();

		// Create SVG
		const svg = d3.select(mapContainer)
			.append('svg')
			.attr('width', '100%')
			.attr('height', '100%')
			.attr('viewBox', `0 0 ${width} ${height}`)
			.attr('preserveAspectRatio', 'xMidYMid meet')
			.style('background', 'transparent');

		// Create projection
		const projection = d3.geoMercator()
			.scale(projectionConfig.scale)
			.center(projectionConfig.center)
			.translate([width / 2, height / 2]);

		const path = d3.geoPath().projection(projection);

		// Convert TopoJSON to GeoJSON
		const geojson = topojson.feature(topoData, topoData.objects.india);

		// Create data lookup
		const dataMap = new Map();
		data.forEach(d => {
			dataMap.set(d.id, d);
			dataMap.set(d.state, d);
		});

		// State name to ID mapping
		const stateVariations = {
			'Andhra Pradesh': 'AP',
			'Arunachal Pradesh': 'AR',
			'Assam': 'AS',
			'Bihar': 'BR',
			'Chhattisgarh': 'CT',
			'Gujarat': 'GJ',
			'Haryana': 'HR',
			'Himachal Pradesh': 'HP',
			'Jharkhand': 'JH',
			'Karnataka': 'KA',
			'Kerala': 'KL',
			'Madhya Pradesh': 'MP',
			'Maharashtra': 'MH',
			'Odisha': 'OR',
			'Punjab': 'PB',
			'Rajasthan': 'RJ',
			'Tamil Nadu': 'TN',
			'Uttar Pradesh': 'UP',
			'West Bengal': 'WB',
			'Andaman & Nicobar Island': 'AN',
			'Delhi': 'DL',
			'Goa': 'GA',
			'Jammu & Kashmir': 'JK',
			'Ladakh': 'LA',
			'Lakshadweep': 'LD',
			'Manipur': 'MN',
			'Meghalaya': 'ML',
			'Mizoram': 'MZ',
			'Nagaland': 'NL',
			'Puducherry': 'PY',
			'Sikkim': 'SK',
			'Tripura': 'TR',
			'Uttarakhand': 'UT'
		};

		function getStateData(stateName) {
			let stateData = dataMap.get(stateName);
			if (!stateData) {
				const stateId = stateVariations[stateName];
				if (stateId) {
					stateData = dataMap.get(stateId);
				}
			}
			return stateData;
		}

		// Draw states
		svg.selectAll('.state')
			.data(geojson.features)
			.enter()
			.append('path')
			.attr('class', 'state')
			.attr('d', path)
			.attr('fill', d => {
				const stateData = getStateData(d.properties.name);
				return stateData ? colorScale(stateData.pendingInspections) : '#374151';
			})
			.attr('stroke', '#6b7280')
			.attr('stroke-width', 0.5)
			.style('cursor', 'pointer')
			.on('mouseenter', function(event, d) {
				const stateData = getStateData(d.properties.name);
				if (stateData) {
					dispatch('mouseenter', {
						state: stateData.state,
						pendingInspections: stateData.pendingInspections,
						x: event.pageX,
						y: event.pageY
					});
				}
				d3.select(this)
					.attr('stroke', '#ffffff')
					.attr('stroke-width', 2);
			})
			.on('mousemove', function(event) {
				dispatch('mousemove', { x: event.pageX, y: event.pageY });
			})
			.on('mouseleave', function() {
				dispatch('mouseleave');
				d3.select(this)
					.attr('stroke', '#6b7280')
					.attr('stroke-width', 0.5);
			});

		// Add state labels
		svg.selectAll('.state-label')
			.data(geojson.features)
			.enter()
			.append('text')
			.attr('class', 'state-label')
			.attr('x', d => path.centroid(d)[0])
			.attr('y', d => path.centroid(d)[1])
			.attr('text-anchor', 'middle')
			.attr('fill', 'white')
			.attr('font-size', '10px')
			.attr('font-weight', 'bold')
			.text(d => {
				const stateData = getStateData(d.properties.name);
				return stateData ? stateData.pendingInspections : '';
			})
			.style('pointer-events', 'none');
	}

	// Reactive statement to re-render map when data changes
	$: if (mapContainer && topoData && data.length > 0) {
		renderMap();
	}
</script>

<div bind:this={mapContainer} class="w-full h-full"></div>

{#if isLoading}
	<div class="flex items-center justify-center h-64">
		<div class="text-center">
			<div class="mx-auto h-12 w-12 animate-spin rounded-full border-b-2 border-purple-500"></div>
			<p class="mt-4 text-gray-400">Loading map...</p>
		</div>
	</div>
{/if}
