<script lang="ts">
	import ProgressBar from './ProgressBar.svelte';

	import { subscribeToEventBus } from '$lib/backend/playerService';
	import { onMount, onDestroy } from 'svelte';
	import { seek } from '../../backend/commands';

	let progressBar: ProgressBar | null = null;
	let progress = 0;
	let total = 0;
	const unsubscribe = subscribeToEventBus((event) => {
		progress = (event?.percentage ?? 0) * 100;
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
		let time = Math.round((percentage / 100) * total);
		seek(time);
	};
</script>

<div
	class="flex h-20 w-full items-center justify-center border-t border-amptree-border bg-amptree-surface"
>
	<div class="w-1/2">
		<ProgressBar bind:this={progressBar} on:change={handleSeek} />
	</div>
</div>
