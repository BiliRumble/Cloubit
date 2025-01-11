//import { useSettingStore } from '@/store/setting';

import { useSettingStore } from '../store/setting';

/**
 * @description: 全局主题设置
 */
export const useTheme = () => {
	// 获取全局状态管理仓库中系统设置状态
	const settingStore = useSettingStore();

	const switchMode = (type: 'light' | 'dark' | 'auto') => {
		const body = document.documentElement;
		switch (type) {
			case 'light':
				body.setAttribute('data-theme', 'light');
				break;
			case 'dark':
				body.setAttribute('data-theme', 'dark');
				break;
			case 'auto': {
				const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
				if (isDark) body.setAttribute('data-theme', 'dark');
				else body.setAttribute('data-theme', 'light');
				break;
			}
		}
	};

	const initTheme = () => {
		const type = settingStore.theme;
		switchMode(type);
	};

	initTheme();

	const currentTheme = () => {
		return settingStore.theme;
	};

	return {
		switchMode,
		currentTheme,
		initTheme,
	};
};
