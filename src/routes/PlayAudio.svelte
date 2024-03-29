<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	let errorMessage: String | null;

	const playAudio = () => {
		invoke('play_audio', { path: '/home/hammsvietro/test.flac' }).catch((err) => {
			errorMessage = err;
		});
	};
	const pause = () => {
		invoke('pause', { path: '/home/hammsvietro/test.flac' }).catch((err) => {
			errorMessage = err;
		});
	};
	const resume = () => {
		invoke('resume', { path: '/home/hammsvietro/test.flac' }).catch((err) => {
			errorMessage = err;
		});
	};

	const jumpTo = (seconds: number) => {
		invoke('seek', { seconds }).catch((err) => {
			errorMessage = err;
		});
	};
</script>

<button on:click={playAudio}>Play audio</button>
<button on:click={pause}>Pause</button>
<button on:click={resume}>Resume</button>
<button on:click={() => jumpTo(180)}>Jump to</button>

{#if errorMessage != null}
	<div style="color: red">{errorMessage}</div>
{/if}

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
