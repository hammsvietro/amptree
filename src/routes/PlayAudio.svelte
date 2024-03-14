<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	const playAudio = () => {
		invoke('play_audio', { path: '/home/hammsvietro/test.flac' }).catch(console.log);
	};
</script>

<div class="counter">
	<button on:click={playAudio} aria-label="Decrease the counter by one">Play audio</button>
</div>

<style>
	.counter {
		display: flex;
		border-top: 1px solid rgba(0, 0, 0, 0.1);
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
		margin: 1rem 0;
	}

	.counter button {
		width: 2em;
		padding: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 0;
		background-color: transparent;
		touch-action: manipulation;
		font-size: 2rem;
	}

	.counter button:hover {
		background-color: var(--color-bg-1);
	}
</style>
