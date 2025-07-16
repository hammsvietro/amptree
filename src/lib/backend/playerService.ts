import { listen } from '@tauri-apps/api/event';
import { writable } from 'svelte/store';

export interface PlayerEvent {
	percentage?: number;
	playedSecs?: number;
	totalDurationSecs?: number;
	volume?: number;
}

const eventBus = writable<PlayerEvent | null>(null);

function dispatchEvent(event: PlayerEvent) {
	eventBus.set(event);
}

// Function to subscribe to the event bus
export function subscribeToEventBus(callback: (event: PlayerEvent | null) => void) {
	return eventBus.subscribe(callback);
}

export async function listenToServer() {
	await listen('player:tick', (event) => {
		dispatchEvent(event.payload as PlayerEvent);
	});
}
