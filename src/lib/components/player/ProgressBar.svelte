<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';

	let progress = 0;
	const dispatch = createEventDispatcher();

	const accentColor = '#4db8ff';
	const bgColor = '#2d2d2d';

	let slider: HTMLInputElement | null = null;
	let min = 0;
	let max = 100;

	onMount(() => {
		updateSliderBar();
	});

	export function setProgress(value: string | number) {
		progress = parseFloat(value as string);
		updateSliderBar();
	}

	const updateSliderBar = () => {
		slider!.style.background = `
			linear-gradient(
				to right,
				${accentColor} 0%,
				${accentColor} ${((progress - min) / (max - min)) * 100}%,
				${bgColor} ${((progress - min) / (max - min)) * 100}%,
				${bgColor} 100%
			)
		`;
	};

	const handleInputChange = (event: Event) => {
		const target = event.target as HTMLInputElement;
		manualChange(parseFloat(target.value));
	};

	function manualChange(value: number) {
		setProgress(value);
		dispatch('change', value);
	}
</script>

<div class="relative w-full">
	<!-- Hidden range input -->

	<!-- Progress bar container -->
	<div class="flex items-center justify-items-center h-3 border-none">
		<input
			type="range"
			{min}
			{max}
			step="0.01"
			bind:value={progress}
			bind:this={slider}
			on:input={handleInputChange}
		/>
	</div>
</div>
