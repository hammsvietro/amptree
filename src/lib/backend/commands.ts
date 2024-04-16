import { invoke } from '@tauri-apps/api';

export async function playAudio(path: string): Promise<void> {
	return invoke('play_audio', { path });
}

export async function pause(): Promise<void> {
	return invoke('pause');
}
export async function resume(): Promise<void> {
	return invoke('resume');
}

export async function seek(seconds: number): Promise<void> {
	return invoke('seek', { seconds });
}
export async function queue(path: string): Promise<void> {
	return invoke('queue', { path });
}
export async function skip(): Promise<void> {
	return invoke('skip');
}
