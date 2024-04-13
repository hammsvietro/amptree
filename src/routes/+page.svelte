<script>
	import PlayAudio from './PlayAudio.svelte';
	export let data;
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h2>Select your song.</h2>
	<a href="/album">To other page</a>

	<div class="songs text-primary-text-c">
		<ul>
			{#each data.items as item}
				<li>
					<div class="song-id">{item.id}</div>
					<div class="song-name">{item.name}</div>
					<div class="song-length">{(item.length / 60).toFixed(2).replace('.', ':')}</div>
				</li>
			{/each}
		</ul>
	</div>

	<PlayAudio />
</section>
