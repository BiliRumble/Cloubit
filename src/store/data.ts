import { create } from 'zustand';
import { persist } from 'zustand/middleware';

/**
 * 系统设置store类型
 */
export interface dataStoreType {
}

export const useSettingStore = create(
	persist<dataStoreType>(
		(set) => ({
		}),
		{
			name: 'settings-storage',
		}
	)
);
