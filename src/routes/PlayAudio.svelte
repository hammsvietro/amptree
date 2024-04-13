<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { playAudio, pause, resume, seek } from '../lib/backend/commands';
	let errorMessage: String | null;

	const doPlayAudio = () => {
		playAudio('/home/hammsvietro/test.flac').catch((err: string) => {
			errorMessage = err;
		});
	};
	const doPause = () => {
		invoke('pause', { path: '/home/hammsvietro/test.flac' }).catch((err: string) => {
			errorMessage = err;
		});
	};
	const doResume = () => {
		invoke('resume', { path: '/home/hammsvietro/test.flac' }).catch((err: string) => {
			errorMessage = err;
		});
	};

	const doSeek = (seconds: number) => {
		invoke('seek', { seconds }).catch((err: string) => {
			errorMessage = err;
		});
	};
</script>

<button on:click={doPlayAudio}>Play audio</button>
<button on:click={doPause}>Pause</button>
<button on:click={doResume}>Resume</button>
<button on:click={() => doSeek(180)}>Jump to</button>

{#if errorMessage != null}
	<div style="color: red">{errorMessage}</div>
{/if}
