import { create } from 'zustand';
import { persist } from 'zustand/middleware';

/**
 * 系统设置store类型
 */
export interface settingsStoreType {
	theme: 'light' | 'dark' | 'auto';
	setTheme: (value: 'light' | 'dark' | 'auto') => void;
	searchShowHot: boolean;
	setSearchShowHot: (value: boolean) => void;
}

export const useSettingStore = create(
	persist<settingsStoreType>(
		(set) => ({
			theme: 'auto',
			setTheme: (value) => set(() => ({ theme: value })),
			searchShowHot: true,
			setSearchShowHot: (value) => set(() => ({ searchShowHot: value })),
		}),
		{
			name: 'settings-storage', // 存储的名称
		}
	)
);
