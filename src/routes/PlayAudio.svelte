<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { playAudio, pause, resume, seek, queue, skip } from '../lib/backend/commands';
	let errorMessage: String | null;
	const path = '/Users/pedrovietro/Downloads/sample3.flac';

	const doPlayAudio = () => {
		playAudio(path).catch((err: string) => {
			errorMessage = err;
		});
	};
	const doPause = () => {
		invoke('pause', { path }).catch((err: string) => {
			errorMessage = err;
		});
	};
	const doResume = () => {
		invoke('resume', { path }).catch((err: string) => {
			errorMessage = err;
		});
	};

	const doSeek = (seconds: number) => {
		invoke('seek', { seconds }).catch((err: string) => {
			errorMessage = err;
		});
	};
	const doQueue = () => {
		queue(path).catch((err: string) => {
			errorMessage = err;
		});
	};
	const doSkip = () => {
		skip().catch((err: string) => {
			errorMessage = err;
		});
	};
</script>

<button on:click={doPlayAudio}>Play audio</button>
<button on:click={doPause}>Pause</button>
<button on:click={doResume}>Resume</button>
<button on:click={() => doSeek(102)}>Jump to</button>
<button on:click={doQueue}>Queue</button>
<button on:click={doSkip}>Skip</button>

{#if errorMessage != null}
	<div style="color: red">{errorMessage}</div>
{/if}
