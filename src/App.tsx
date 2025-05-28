import { useEffect } from 'react';
import { RouterProvider } from 'react-router-dom';
import { useTheme } from './hooks/useTheme';
import { unregisterAllShortcuts } from './managers/ShortcutManager';
import { useSettingStore } from './store/setting';
import { eventBus } from './utils/EventBus';
import { init } from './utils/init';
import Router from './routers';

const App = () => {
	const isWindowPath = location.pathname.startsWith('/windows/');
	useTheme();

	useEffect(() => {
		if (!isWindowPath) {
			init();
			eventBus.emit('test');
		}
	}, []);

	useEffect(() => {
		return () => {
			unregisterAllShortcuts();
		};
	}, [useSettingStore.getState().enableGlobalShortcut]);
	// invoke('api_search', {
	// 	keyword: 'Echo',
	// 	types: 1,
	// 	limit: 2,
	// 	offset: 0,
	// }).then((res) => {
	// 	console.log(res);
	// });

	return <RouterProvider router={Router} />;
};

export default App;
