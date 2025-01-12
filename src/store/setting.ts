import { create } from 'zustand';
import { persist } from 'zustand/middleware';

/**
 * 系统设置store类型
 */
export interface settingsStoreType {
	theme: 'light' | 'dark' | 'auto';
	setTheme: (value: 'light' | 'dark' | 'auto') => void;
	searchAutoComplete: boolean;
	setSearchAutoComplete: (value: boolean) => void;
	searchHistoryRecord: boolean;
	setSearchHistoryRecord: (value: boolean) => void;
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
		}),
		{
			name: 'settings-storage',
		}
	)
);
