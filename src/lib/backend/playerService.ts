import { listen } from '@tauri-apps/api/event';
import { writable } from 'svelte/store';

const eventBus = writable(null);

function dispatchEvent(event: any) {
	eventBus.set(event);
}

// Function to subscribe to the event bus
export function subscribeToEventBus(callback: (event: any) => void) {
	return eventBus.subscribe(callback);
}

export async function listenToServer() {
	let tickCount = 0;
	await listen('player:tick', (event) => {
		dispatchEvent(event.payload);
	});
}
