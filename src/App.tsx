import { useEffect } from 'react';
import { BrowserRouter } from 'react-router-dom';
import Navbar from './components/templates/Navbar';
import PlayBar from './components/templates/PlayBar';
import Sidebar from './components/templates/Sidebar';
import { useTheme } from './hooks/useTheme';
import { unregisterAllShortcuts } from './managers/ShortcutManager';
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
		return () => {
			unregisterAllShortcuts();
		};
	}, [useSettingStore.getState().enableGlobalShortcut]);

	return (
		<BrowserRouter>
			{!isWindowPath ? (
				<main className={styles.layout}>
					<Sidebar className={styles.layout__sidebar} />
					<div className={styles.layout__content}>
						<Navbar className={styles.layout__content__header} />
						<main className={styles.layout__content__body}>
							<Router />
						</main>
					</div>
					<PlayBar className={styles.layout__playbar} />
				</main>
			) : (
				<Router />
			)}
		</BrowserRouter>
	);
};

export default App;
