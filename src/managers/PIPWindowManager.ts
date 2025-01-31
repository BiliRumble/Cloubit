import { LogicalSize } from '@tauri-apps/api/dpi';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Window } from '@tauri-apps/api/window';

const appWindow = new Window('main');

export async function createPip(init: boolean = false) {
	if (!init && localStorage.getItem('isPip') === 'true') return;

	try {
		const pipWindow = new WebviewWindow('pip-window', {
			url: '/windows/pip',
			minWidth: 325,
			minHeight: 90,
			maxWidth: 325,
			maxHeight: 90,
			transparent: true,
			alwaysOnTop: true,
			decorations: false,
			skipTaskbar: true,
			resizable: false,
			minimizable: false,
			shadow: false,
		});

		pipWindow.once('tauri://created', () => {
			pipWindow.setSize(new LogicalSize(325, 90));
			if (!init) appWindow.hide();
			localStorage.setItem('isPip', 'true');
		});
		console.debug('🪟 PIPWindow created.');
	} catch (error) {
		console.error('🪟 Cannot create PIPWindow:', error);
	}
}

export async function closePip() {
	try {
		WebviewWindow.getByLabel('pip-window').then((pipWindow) => {
			appWindow.show();
			pipWindow?.destroy();
		});
		localStorage.removeItem('isPip');
	} catch (error) {
		console.error('🪟 Cannot close PIPWindow:', error);
	}
}
