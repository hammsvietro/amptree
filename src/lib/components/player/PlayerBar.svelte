<script lang="ts">
	import ProgressBar from './ProgressBar.svelte';

	import { subscribeToEventBus } from '$lib/backend/playerService';
	import { onMount, onDestroy } from 'svelte';
	import { seek } from '../../backend/commands';

	let progressBar: ProgressBar | null = null;
	let progress = 0;
	let current = 0;
	let total = 0;
	const unsubscribe = subscribeToEventBus((event) => {
		progress = (event?.percentage ?? 0) * 100;
		current = event?.playedSecs;
		total = event?.totalDurationSecs;
		if (progressBar == null) return;
		progressBar.setProgress(progress);
	});

	onMount(() => {
		if (progressBar == null) return;
		progressBar.setProgress(progress);
	});
	onDestroy(() => {
		console.log('unsubbing');
		unsubscribe();
	});

	const handleSeek = (event: CustomEvent) => {
		let percentage = event.detail;
		let time = (percentage / 100) * total;
		seek(parseInt(time));
	};
</script>

<div
	class="w-full bg-amptree-surface h-20 border-t border-amptree-border flex justify-center items-center"
>
	<div class="w-1/2">
		<ProgressBar bind:this={progressBar} on:change={handleSeek} />
	</div>
</div>
