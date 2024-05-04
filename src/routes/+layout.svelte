<script>
	import '../app.pcss';
	import PlayerBar from '../lib/components/player/PlayerBar.svelte';
	import { listenToServer } from '../lib/backend/playerService';
	import { onMount } from 'svelte';

	onMount(() => {
		console.log('mount!');
		listenToServer();
	});
</script>

<main
	class="flex min-h-screen flex-col justify-between bg-amptree-bg leading-tight text-amptree-text antialiased"
>
	<div class="grow">
		<slot />
	</div>
	<PlayerBar></PlayerBar>
</main>
