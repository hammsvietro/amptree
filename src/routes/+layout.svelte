<script>
	import '../app.css';
	import PlayerBar from '../lib/components/PlayerBar.svelte';
</script>

<main
	class="min-h-screen bg-slate-800 text-white antialiased leading-tight flex flex-col justify-between"
>
	<div class="grow">
		<slot />
	</div>
	<PlayerBar />
</main>
