<script>
	import '../app.css';
	import PlayerBar from '../lib/components/player/PlayerBar.svelte';
	import { listenToServer } from '../lib/backend/playerService';
	import { onMount } from 'svelte';

	onMount(() => {
		console.log('mount!');
		listenToServer();
	});
</script>

<main
	class="min-h-screen bg-amptree-bg text-amptree-text antialiased leading-tight flex flex-col justify-between"
>
	<div class="grow">
		<slot />
	</div>
	<PlayerBar />
</main>
