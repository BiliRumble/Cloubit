import { event } from '@tauri-apps/api';
import { useEffect } from 'react';
import { BrowserRouter } from 'react-router-dom';
import Navbar from './components/Layout/Navbar';
import PlayBar from './components/Layout/PlayBar';
import Sidebar from './components/Layout/Sidebar';
import { useTheme } from './hooks/useTheme';
import { registerShortcuts, unregisterAllShortcuts } from './managers/ShortcutManager';
import { useSettingStore } from './store/setting';
import { init } from './utils/init';
import Router from './routers';
import styles from './styles/layout.module.scss';

const App = () => {
	const isWindowPath = location.pathname.startsWith('/windows/');
	useTheme();

	useEffect(() => {
		init();
	}, []);

	useEffect(() => {
		unregisterAllShortcuts();
		if (!useSettingStore.getState().enableGlobalShortcut) return;
		// 注册快捷键
		registerShortcuts(useSettingStore.getState().playShortcut.join('+'), () =>
			event.emit('shortcut-play')
		);
		registerShortcuts(useSettingStore.getState().prevShortcut.join('+'), () =>
			event.emit('shortcut-prev')
		);
		registerShortcuts(useSettingStore.getState().nextShortcut.join('+'), () =>
			event.emit('shortcut-next')
		);
		registerShortcuts(useSettingStore.getState().volumeUpShortcut.join('+'), () =>
			event.emit('shortcut-volumeUp')
		);
		registerShortcuts(useSettingStore.getState().volumeDownShortcut.join('+'), () =>
			event.emit('shortcut-volumeDown')
		);

		return () => {
			unregisterAllShortcuts();
		};
	}, [useSettingStore.getState().enableGlobalShortcut]);

	return (
		<BrowserRouter>
			{!isWindowPath ? (
				<main className={styles.layout}>
					<Sidebar className={styles.sidebar} />
					<div className={styles.content}>
						<Navbar className={styles.header} />
						<main className={styles.page_content}>
							<Router />
						</main>
					</div>
					<PlayBar className={styles.bottom_bar} />
				</main>
			) : (
				<Router />
			)}
		</BrowserRouter>
	);
};

export default App;
