import { event } from '@tauri-apps/api';
import { debounce } from 'lodash-es';
import { getUserAccount } from '../apis/user';
import { registerShortcuts, unregisterAllShortcuts } from '../managers/ShortcutManager';
import { useAuthStore } from '../store/auth';
import { useSettingStore } from '../store/setting';
export async function init() {
	const isLogin = useAuthStore.getState().isLogin;

	if (isLogin) {
		await getUserAccount().then((res) => {
			useAuthStore.setState({ userData: res });
		});
	}

	registerShortcut();
}

export function disableShortcuts() {
	const shortcuts = [
		{ key: 'F5', modifiers: [] },
		{ key: 'r', modifiers: ['Ctrl'] },
		{ key: 'f', modifiers: ['Alt'] },
		{ key: 'F11', modifiers: [] },
		{ key: 'f', modifiers: ['Ctrl'] },
		{ key: 'p', modifiers: ['Ctrl'] },
		{ key: 's', modifiers: ['Ctrl'] },
		{ key: 'd', modifiers: ['Ctrl'] },
		{ key: 't', modifiers: ['Ctrl'] },
		{ key: 'w', modifiers: ['Ctrl'] },
	];

	window.addEventListener('keydown', (e) => {
		const pressedModifiers: string[] = [];
		if (e.ctrlKey) pressedModifiers.push('Ctrl');
		if (e.altKey) pressedModifiers.push('Alt');
		if (e.shiftKey) pressedModifiers.push('Shift');

		const currentShortcut = shortcuts.find(
			(s) => s.key === e.key && arraysEqual(s.modifiers, pressedModifiers)
		);

		if (currentShortcut) {
			e.preventDefault();
		}
	});
}

function arraysEqual(a: string[], b: string[]): boolean {
	if (a.length !== b.length) return false;
	for (let i = 0; i < a.length; i++) {
		if (a[i] !== b[i]) return false;
	}
	return true;
}

function registerShortcut() {
	unregisterAllShortcuts();
	if (!useSettingStore.getState().enableGlobalShortcut) return;

	const playShortcut = useSettingStore.getState().playShortcut.join('+');
	const prevShortcut = useSettingStore.getState().prevShortcut.join('+');
	const nextShortcut = useSettingStore.getState().nextShortcut.join('+');
	const volumeUpShortcut = useSettingStore.getState().volumeUpShortcut.join('+');
	const volumeDownShortcut = useSettingStore.getState().volumeDownShortcut.join('+');

	registerShortcuts(
		playShortcut,
		debounce(() => event.emit('shortcut-play'), 10)
	);
	registerShortcuts(
		prevShortcut,
		debounce(() => event.emit('shortcut-prev'), 10)
	);
	registerShortcuts(
		nextShortcut,
		debounce(() => event.emit('shortcut-next'), 10)
	);
	registerShortcuts(
		volumeUpShortcut,
		debounce(() => event.emit('shortcut-volumeUp'), 10)
	);
	registerShortcuts(
		volumeDownShortcut,
		debounce(() => event.emit('shortcut-volumeDown'), 10)
	);
}
