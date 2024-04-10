<script>
	import { onMount } from 'svelte';

	let progress = 0;

	onMount(() => {
		const interval = setInterval(() => {
			progress += 10;
			if (progress >= 100) clearInterval(interval);
		}, 1000);
	});
</script>

<div class="w-full border border-black bg-gray-200 rounded-full h-3 dark:bg-gray-700">
	<div class="bg-blue-600 h-2.5 rounded-full" style="width: {progress}%"></div>
</div>
