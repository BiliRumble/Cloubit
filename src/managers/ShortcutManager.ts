import {
	isRegistered,
	register,
	unregister,
	unregisterAll,
} from '@tauri-apps/plugin-global-shortcut';

// ShortcutManager.ts
export async function registerShortcuts(shortcut: string | string[], callback: () => void) {
	if (Array.isArray(shortcut)) {
		shortcut.join('+');
	}
	try {
		console.debug(`⌨️ Registering shortcut ${shortcut}`);
		await register(shortcut, callback);
	} catch (error) {
		console.error(`Failed to register shortcut ${shortcut}:`, error);
	}
}

export async function unregisterShortcuts(shortcut: string | string[]) {
	try {
		await unregister(shortcut);
		console.debug(`⌨️ Shortcut ${shortcut} unregistered`);
	} catch (error) {
		console.error(`Failed to unregister shortcut ${shortcut}:`, error);
	}
}

export async function unregisterAllShortcuts() {
	try {
		await unregisterAll();
		console.debug(`⌨️ All shortcuts unregistered!`);
	} catch (error) {
		console.error('Failed to unregister all shortcuts:', error);
	}
}

export async function getShortcuts(shortcut: string): Promise<boolean> {
	const isRegisteredShortcut = await isRegistered(shortcut);
	console.debug(
		`⌨️ Shortcut ${shortcut} is ${isRegisteredShortcut ? 'registered' : 'unregistered'}`
	);
	return isRegisteredShortcut || true;
}
