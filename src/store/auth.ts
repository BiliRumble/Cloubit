import { create } from 'zustand';
import { persist } from 'zustand/middleware';

/**
 * 系统设置store类型
 */
export interface authStoreType {
	isLogin: boolean;
	setIsLogin: (isLogin: boolean) => void;
	cookie: string;
	setCookie: (cookie: string) => void;
	userData: any;
	setUserData: (userData: any) => void;
}

export const useAuthStore = create(
	persist<authStoreType>(
		(set) => ({
			isLogin: false,
			setIsLogin: (isLogin) => set({ isLogin }),
			cookie: '',
			setCookie: (cookie) => set({ cookie }),
			userData: {},
			setUserData: (userData) => set({ userData }),
		}),
		{
			name: 'settings-storage',
		}
	)
);
