<script>
	import Counter from './Counter.svelte';
	import PlayAudio from './PlayAudio.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcome_fallback from '$lib/images/svelte-welcome.png';
	export let data;
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			<picture>
				<source srcset={welcome} type="image/webp" />
				<img src={welcome_fallback} alt="Welcome" />
			</picture>
		</span>

		to your new<br />SvelteKit app
	</h1>

	<h2>
		Select your Fucking Song.
	</h2>

	<div class="songs">
		<ul>
			{#each data.items as item}
				<li>
					<div class="song-id">{item.id}</div>
					<div class="song-name">{item.name}</div>
					<div class="song-length">{(item.length / 60).toFixed(2).replace('.',':')}</div>
				</li>
			{/each}
		</ul>
	</div>

	<Counter />
	<PlayAudio />
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>
