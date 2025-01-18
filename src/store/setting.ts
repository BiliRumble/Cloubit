import { create } from 'zustand';
import { persist } from 'zustand/middleware';

/**
 * 系统设置store类型
 */
export interface settingsStoreType {
	theme: 'light' | 'dark' | 'auto';
	setTheme: (value: 'light' | 'dark' | 'auto') => void;

	// 搜索相关
	searchAutoComplete: boolean;
	setSearchAutoComplete: (value: boolean) => void;
	searchHistoryRecord: boolean;
	setSearchHistoryRecord: (value: boolean) => void;

	// 播放器相关
	autoPlay: boolean;
	setAutoPlay: (value: boolean) => void;
	savePlaySeek: boolean;
	setSavePlaySeek: (value: boolean) => void;
	fadeInOut: boolean;
	setFadeInOut: (value: boolean) => void;
	fadeTime: number;
	setFadeTime: (value: number) => void;
	pushToSMTC: boolean;
	setPushToSMTC: (value: boolean) => void;
}

export const useSettingStore = create(
	persist<settingsStoreType>(
		(set) => ({
			theme: 'auto',
			setTheme: (value) => set(() => ({ theme: value })),
			searchAutoComplete: true,
			setSearchAutoComplete: (value) => set(() => ({ searchAutoComplete: value })),
			searchHistoryRecord: true,
			setSearchHistoryRecord: (value) => set(() => ({ searchHistoryRecord: value })),
			autoPlay: false,
			setAutoPlay: (value) => set(() => ({ autoPlay: value })),
			savePlaySeek: true,
			setSavePlaySeek: (value) => set(() => ({ savePlaySeek: value })),
			fadeInOut: true,
			setFadeInOut: (value) => set(() => ({ fadeInOut: value })),
			fadeTime: 1500,
			setFadeTime: (value) => set(() => ({ fadeTime: value })),
			pushToSMTC: true,
			setPushToSMTC: (value) => set(() => ({ pushToSMTC: value })),
		}),
		{
			name: 'settings-storage',
		}
	)
);
