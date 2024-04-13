<script>
	import { subscribeToEventBus } from '$lib/backend/playerService';
	import { onDestroy } from 'svelte';

	let progress = 0;
	const unsubscribe = subscribeToEventBus((event) => {
		// React to events
		console.log('Component A received event:', event);
		const tickCount = event?.tickCount;
		if (tickCount && tickCount > 0 && tickCount <= 100) {
			progress = event.tickCount;
		}
	});

	onDestroy(() => {
		console.log('unsubbing');
		unsubscribe();
	});
</script>

<div
	class="w-full bg-amptree-bg rounded-full h-2 active:h-3 focus:h-3 hover:h-3 mb-4 dark:bg-amptree-bg transition-all transition-[height]"
>
	<div
		class="bg-amptree-accent h-full rounded-full dark:amptree-accent transition-all transition-[width]"
		style="width: {progress}%"
	></div>
</div>
