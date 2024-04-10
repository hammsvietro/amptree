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
