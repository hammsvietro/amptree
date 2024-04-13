<script>
	import { subscribeToEventBus } from '$lib/backend/playerService';
	import { onDestroy } from 'svelte';

	let progress = 0;
	let current = 0;
	let total = 0;
	const unsubscribe = subscribeToEventBus((event) => {
		progress = event?.percentage * 100;
		current = event?.playedSecs;
		total = event?.totalDurationSecs;
	});

	onDestroy(() => {
		console.log('unsubbing');
		unsubscribe();
	});
</script>

<p>{current}</p>
<div
	class="w-full bg-amptree-bg rounded-full h-2 active:h-3 focus:h-3 hover:h-3 mb-4 dark:bg-amptree-bg transition-all transition-[height]"
>
	<div
		class="bg-amptree-accent h-full rounded-full dark:amptree-accent transition-all transition-[width]"
		style="width: {progress}%"
	></div>
</div>

<p>{total}</p>
